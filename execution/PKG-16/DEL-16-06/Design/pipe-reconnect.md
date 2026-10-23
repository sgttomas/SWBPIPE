# Pipe reconnect (W-10, FR-GUI-004)

**Basis:** `origin/main` at `356a9be374` (2026-10-23); W-13 branches
`origin/claude/piping-model-07-s1b-20261023` (`b203794630`) and
`origin/claude/piping-model-states-20261023` (`f0cbc15bb6`). This is a design;
nothing is implemented yet. Paths are relative to `projects/chirality-piping/`.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

FR-GUI-004 (Must) asks for add, modify, delete, move and reconnect while schema
validation is preserved. Everything except reconnect exists (completeness map,
FR-GUI-004 row). A pipe's `from`/`to` is read-only in the table
(`apps/desktop/src/features/model-tree/ModelTree.tsx:1281-1282`), and it is not
a field rule of `Element` (`core/model_operations/operation_applier/src/lib.rs:408-470`).
This deliverable owns the operation applier (`deliverable.yaml` code path
`core/model_operations/operation_applier/**`). DEL-07-01 owns the geometry-tools
UI that drives it. W-10 also fixes a `delete_pipe_run` defect that leaves
components and wind exposure naming a deleted pipe (§7).

## 1. The operation

**Name.** `reconnect_pipe_run`, with operation kind `modify`. This puts it in the
same row as `split_pipe_run` and `transform_pipe_run` (`lib.rs:2450-2452`).

**Wire.** The wire is the same as the geometry wire (`geometry_operations.rs:668-677`):
`target` = (`Element`, pipe id), `field_path` `pipe_segments`, unit `none`,
dimension `dimensionless`. `change.before` is the canonical JSON of the current
pipe record, as for split (`geometry_operations.rs:370`). `change.after` is a
JSON payload:

```json
{ "pipe_ref": "pipe:P-3", "end": "to", "target_node_ref": "node:N-9",
  "y_reference": { "x": 0, "y": 0, "z": 1 } }
```

- `pipe_ref` must equal `target.ref`.
- `end` is `from` or `to`. The other end does not move. The pipe keeps its
  from→to sense.
- `target_node_ref` names an existing node.
- `y_reference` is optional. When it is present it replaces the pipe's
  `y_reference` in the same change. When it is absent the pipe keeps its own.
  The applier never derives an orientation.
- Unknown fields are refused, as `fields()` does today (`geometry_operations.rs:27-41`).

**No new-node mode.** To reconnect to a new point, the desktop sends the batch
[`create_node`, `reconnect_pipe_run`]. The atomic batch validates each
intermediate state against a fresh hash (`atomic_batch.rs:118-125`), so the
operation itself stays small.

**Identity (FR-MOD-003).** No id changes: not the pipe's, the nodes' or any
attachment's. The pipe record is replaced in place, so its array order is kept.
The only fields written are the moved end and, optionally, `y_reference`.

**Fit in the applier.** The code lives in a new module,
`operation_applier/src/pipe_reconnect.rs`, called only from the common applier:
- `check_kinds` gains the kind (`lib.rs:2404-2486`).
- `run_with_context` dispatches next to the geometry branch (`lib.rs:1383-1410`).
  It requires claimed model-hash evidence, as split and transform do
  (`OP-GEOMETRY-MODEL-HASH-REQUIRED`, `lib.rs:1384-1396`), because the checks
  read the pipe's neighbours.
- The resolved candidate model joins the apply chain at `lib.rs:2025-2028`.
  Warnings go through `checker.push(.., "warning", ..)`, as connect does for
  `OP-MASS-NOT-SOLVE-READY` (`lib.rs:3763-3771`).
- The module reuses `record_shape`, `unique`, `frame`, `references`, `diff` and
  `global_ids` from `geometry_operations.rs` (`:118,128,233,318,337,195`). The
  three that are private become `pub(crate)`.

**Diff, validate, undo.** One diff row (`pipe`, `Element`, `pipe_segments`,
before record, after record, `reconnect_pipe_run`), the shape split uses
(`geometry_operations.rs:409-417`). Validate mode returns diff and diagnostics,
never a model (`lib.rs:885-905`). Desktop undo/redo holds whole-model
checkpoints per applied batch (`apps/desktop/src/features/workspace/workspaceSession.ts:1192-1198`),
so reconnect needs no new undo code. The desktop runs the same crate as wasm
(`apps/desktop/src/services/operationService.ts:14`); there is no TypeScript
mirror.

**Agent proposals.** Offline intake reaches the same batch path
(`atomic_batch.rs:285-345`). Reconnect writes no user-only datum (unlike k,
`lib.rs:1366-1381`), so an agent may propose it and the user reviews and applies
it as any batch. `schemas/model_operation.schema.json`, which the intake panel
hands agents as the capability reference (`App.tsx:509`), already lists a
generic `reconnect` (`:346,421`); its `change_kind` enum gains
`reconnect_pipe_run` as it gained split and transform (`:337-338`).

## 2. Topology rules and refusals

**Basis for the rules.**
- **Moving a node.** A node move (`Node` `position.*`, `lib.rs:386-406`) already
  changes the length and direction of every incident pipe through the generic
  field path, and it checks no attachment. So a geometry consequence alone is
  not a reason to refuse. The applier warns where a preserved attachment's
  physical extent changes.
- **Changing connectivity.** Reconnect also changes which pipes meet at a node.
  The applier refuses wherever that breaks a declared relationship. It follows
  split's precedent (`attachments`, `geometry_operations.rs:251-317`): any
  component that names the pipe or an affected node blocks.
- **Unknown references.** Every reference outside the slots understood below
  fails closed, as `references()` does (`:231-250`).

The affected nodes are the old end node *O* and the target node *T*. Each
refusal is blocking and leaves the model unchanged.

| # | Condition | Result | Code |
|---|---|---|---|
| R1 | Pipe absent, or `pipe_ref` ≠ target | refuse | `OP-TARGET-NOT-FOUND` / `OP-RECONNECT-PAYLOAD-INVALID` |
| R2 | `before` ≠ current canonical pipe | refuse | `OP-STALE-BEFORE-VALUE` (`lib.rs:7608-7628`) |
| R3 | *T* absent | refuse | `OP-RECONNECT-TARGET-NODE-NOT-FOUND` |
| R4 | *T* = *O* | refuse | `OP-RECONNECT-NO-CHANGE` |
| R5 | *T* = the fixed end node (zero length) | refuse | `OP-RECONNECT-ZERO-LENGTH` |
| R6 | *T* coincides in position with the fixed end, or the axis is degenerate | refuse | `OP-RECONNECT-GEOMETRY-DEGENERATE` (`frame`, `:318-336`; 1e-12, the frame kernel's `AXIS_TOLERANCE`, `frame_kernel/src/lib.rs:27`) |
| R7 | The kept or entered `y_reference` is parallel to the new axis | refuse; enter `y_reference` in the payload | `OP-RECONNECT-ORIENTATION-DEGENERATE` (frame kernel `:627`) |
| R8 | Another pipe already joins the same unordered node pair | refuse | `OP-RECONNECT-PARALLEL-PIPE` |
| R9 | Any component names the pipe (`bend_pipe_ref`, tee header/branch, `rigid_pipe_ref`, `expansion_joint_pipe_ref`, connector `replaces_span`, SIF modifiers), or any component sits at *O* or *T* (`component.node`) | refuse; re-author the component | `OP-RECONNECT-COMPONENT-ATTACHED` |
| R10 | The pipe is a member of any pressure region in any case (`member_pipe_ids`) | refuse; re-author the region | `OP-RECONNECT-PRESSURE-REGION-MEMBER` |
| R11 | *T* is an internal node of a region (an end of a member pipe or connector span that is not a terminal) | refuse | `OP-RECONNECT-PRESSURE-REGION-BRANCH` |
| R12 | Any other reference to the pipe id outside the slots in §2.2 | refuse; list the paths | `OP-RECONNECT-ATTACHMENT-UNSUPPORTED` |

### 2.1 Why these refusals

- **R8, parallel pipes.** Neither the solver nor `connect_pipe_run` checks pairs.
  The product build adds each pipe as its own element (`product_physics/src/lib.rs:7980-8170`),
  so a coincident pair silently doubles stiffness and weight. In one region the
  pair makes a two-edge cycle, and the solve refuses it as
  `PRESSURE_REGION_TOPOLOGY_INVALID` (`pressure_runtime.rs:1535-1540,1594-1596`).
  A pair outside a region that touches its internal node is
  `PRESSURE_REGION_INTERNAL_BRANCH` (`:1541-1549`). The viewport would also draw
  two pipes on top of each other. The pair has no modelling use that a single
  pipe with the combined section does not serve better. Reconnect refuses it.
  `connect_pipe_run` still admits it (`lib.rs:3667-3686`); see §6.
- **R9, components.** A bend's arc is formed from the span's chord, radius and
  `y_reference` (`bend_authoring.rs:109-124`), and the authored angle must match
  that arc. A tee must keep its node at an end of both legs
  (`referenced_pipe_is_incident`, `lib.rs:2831-2869`), and it carries a
  user-entered connection angle. A connector carries an explicit frame. Any of
  these goes stale when the span moves, so a component must be re-authored. A
  component at *O* or *T* blocks for the same reason split blocks one at its
  endpoints.
- **R10–R11, pressure regions.** Region chain topology is checked only at solve,
  in `traverse_region` (`pressure_runtime.rs:1458-1597`). The authoring check
  (`pressure_authoring.rs:141-221`) covers members, terminals and bend
  tangency, but not degree, branching or cycles. `pressure_runtime.rs` is in the
  D3-pinned set (`tools/retained_pricing/base/b7a_domain_model.py:74`). So the
  applier cannot re-run the chain check without either duplicating pinned logic
  or editing a pinned file. Instead it refuses:
  - any reconnect of a member pipe;
  - any move of an end onto an internal node, which the solve would refuse as
    `PRESSURE_REGION_INTERNAL_BRANCH`.

  Moving a non-member pipe away from an internal node removes a branch, so it is
  allowed. Region membership and closure are never inferred (`:1548`).

### 2.2 What is preserved, and the warnings

| Attached to | Slot | Kept as is | Warning |
|---|---|---|---|
| The pipe | element primitive loads, `target.pipe`, distributed force or thermal (`lib.rs:5422-5423`); generated self-weight included | yes, with the same id and magnitude per length in the global direction | `OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED` (distributed force only): L_old → L_new, so the total changes. Self-weight staleness compares section-only dependencies (`self_weight.rs:375,941`), so a generated record is expected to stay Fresh; T13 pins it |
| The pipe | wind `exposed_pipe_refs`, `exposed_spans[].pipe_ref` with fractions (`lib.rs:829`; schema `WindExposedSpan`) | yes | `OP-RECONNECT-WIND-EXPOSURE-REMAPPED`: generated wind follows the new geometry, and fractions apply to the new length |
| The pipe | `analysis_state.element_states[].pipe_ref` (`load_state_authoring.rs:368-377`) | yes | none (member properties) |
| The pipe | `reference_configurations[].member_references[].pipe_ref` (`:344-367`) | yes | `OP-RECONNECT-MEMBER-FIT-REVIEW` when `fit` is not `none`. The natural-length reference is the authored chord (schema `FitReference`), so a declared length change or strain now applies to a member with a new chord |
| The pipe | `lines[].pipe_refs` (0.7.0, §5), and assumptions or constraints naming the pipe | yes; membership is by id and there is no contiguity rule | none |
| *O*, *T* | supports, linear and nonlinear (DOFs are global: schema `NonlinearSupport`), nodal loads, support and boundary states, equipment interfaces | yes, they stay on their node | see orphan and split below |
| *O*, *T* | region terminals (`terminals[].node_ref`) | yes | `OP-RECONNECT-PRESSURE-TERMINAL-REVIEW`: an external pipe now joins, or leaves, a terminal whose `closure_transfer` the user declared |

**Topology warnings** (non-blocking; the user may be mid-edit):
- **`OP-RECONNECT-NODE-ORPHANED`.** *O* has no pipe left. The node is kept with
  its attachments, and the warning lists them. An unconnected, unrestrained node
  is expected to make the solve report a mechanism. The nearest existing witness
  is the disconnected-body test (`product_physics/src/lib.rs:15409-15434`);
  T10 pins it for this case. The desktop offers to delete *O* in the same batch
  (§3). The applier does not delete it implicitly.
- **`OP-RECONNECT-MODEL-SPLIT`.** The pipe graph has more connected parts than
  before. A part with no restraint does not solve (same witness). A cycle is
  allowed with no diagnostic, because loops are ordinary piping.
- **`OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT`.** *T* now joins three or
  more pipes and has no component (R9 guarantees none). No tee is modelled
  there. On an exact document, admission is by component kind
  (`exact_admission.rs:205-206,284`), so this junction is not refused there
  either.
- **`OP-RECONNECT-BEND-JUNCTION-NOT-TANGENT`.** *T* is an end of a realized bend
  span, and the new member meets it at θ > α_tan. θ comes from `junction`
  (`bend_authoring.rs:154-158`, through a new `pub(crate)` helper). A region
  that later includes both members is refused at authoring or solve as a mitre
  (`bend_authoring.rs:201-210`, `pressure_runtime.rs:1679-1682`).

**Geometry consequences that need no diagnostic.**
- The local axes follow the new axis and the kept `y_reference`, as after a
  node move.
- Results in local axes change orientation accordingly.
- Thermal expansion scales with the new length through the element.
- Bend tangency is not enforced outside regions.

## 3. Desktop UX

**Where.** Geometry tools gains a `reconnect` action
(`apps/desktop/src/features/geometry-tools/GeometryToolsPanel.tsx:59`,
`geometryDraft.ts:10`) with: one source pipe (existing picker and "Use selected
pipes"); `End` (`from` | `to`, each showing its current node); `Target node`
(picker, or a new "Use selected node"); optional `y_reference`. "To a new
point" adds the `create_node` fields and queues the two-operation batch. The
panel states the §2 rules in one paragraph, as split/transform do (`:56`). The
toolkit gains `build.reconnect`, routed to `geometry-tools`
(`features/toolkit/capabilityCatalog.ts:63-71,390`).

**Not in v1.** No viewport handle drag or snap: viewport drag orbits the view
today (`PipeViewport.tsx:2388,2554`) and node picking exists only inside the
create-pipe draft (`:1703-1716,1947-1950`). From/To stay read-only in the table:
a cell edit would bypass the batch review that shows the warnings, and S2 is
editing the model tree. Either can follow on the same operation.

**Orphan cleanup.** If *O* would be left with no pipe and no other reference
(`node_references`, `lib.rs:8618-8700`), the panel offers "Also delete node *O*"
(unchecked), which appends `delete_node` to the same batch; `delete_node`
refuses any remaining reference itself (`lib.rs:3404-3508`).

**Preview, refusal, undo.** "Queue reconnect batch" uses the existing batch
review, validate then a separate Apply (`workspaceSession.ts:1170-1198`). The
review shows the diff row and every warning; a refusal shows code, message and
refs with the model unchanged. Undo/redo restore the whole-model checkpoint.
During review the viewport highlights the pipe, *O* and *T* through the
existing selection presentation; no ghost geometry.

## 4. Tests

**Applier** (`tests/pipe_reconnect.rs`; inline models built from
`MODEL_SCHEMA_VERSION`, never a version literal):
- T1–T12, one per rule R1–R12: the code, unchanged model, empty diff, and no
  model from validate mode.
- T13: each §2.2 slot survives byte-identically; each warning appears exactly
  when its condition holds.
- T14 identity: id sets and array order unchanged; only the moved end and the
  optional `y_reference` differ.
- T15: agent and user authors give the same outcome; hash required; a stale
  hash refused.
- Property-style (deterministic seeds, no new dependency): random graphs of 3–12
  nodes, every (pipe, end, target). Each outcome is a refusal with no mutation,
  or an application with all endpoints existing, no zero-length pipe, no new
  parallel pair, id sets unchanged and undo restoring an equal hash. Warning
  presence matches an independent recomputation of the orphan, split and degree
  conditions.
- Batches: [`create_node`, `reconnect`] and [`reconnect`, `delete_node`] apply
  atomically; a late refusal rolls back (pattern `atomic_batch.rs:609-652`).

**Numerical equivalence** (after `tests/l_line_authoring.rs:154-200`). For an L
line and a small supported tree with an element distributed load, author model A
directly and model B by connecting the pipe elsewhere and then reconnecting it.
Assert canonical(A) = canonical(B), then solve both in `DenseScrutiny` and
`SparseInteractive` and assert equal result rows bit for bit. Repeat on an
exact-pressure document with a non-member pipe. T10: orphaning an unsupported
node gives a mechanism status, not a result.

**Contract corpus** (native and wasm): one accept case and two block cases (R9,
R10), blessed with `CORPUS_BLESS=1`, each needing its own human review entry
(corpus README), listed under `contract_corpus.rs` in
`tools/hosted-ci-routing.json:431-`.

**Desktop and e2e.** `geometryDraft.test.ts` (payload, two-operation batch),
`GeometryToolsPanel.test.tsx` (fields, "Use selected node", orphan checkbox,
refusal display), `ToolkitPalette.test.tsx` (capability). New
`apps/desktop/e2e/pipe-reconnect.spec.ts` (after
`b4-table-editing.spec.ts:7-50`): reconnect through Geometry tools, review,
apply, check From/To in the table, undo, redo; then a component-attached pipe
shows the refusal and the model is unchanged.

## 5. Overlap with W-13 S1b and S2

**Branch facts.**
- **S1b.** `origin/claude/piping-model-07-s1b-20261023` changes 423 files:
  - in the applier, only version literals (`geometry_operations.rs:661-667,693`;
    `lib.rs:2677-2810,5169,8951,13437-13457`);
  - the regenerated contract corpus (all 84 cases);
  - fixtures;
  - the D3 chain.
- **States branch.** `origin/claude/piping-model-states-20261023` differs from
  main only in the design document (+5 lines).
- **S2.** Not written yet. Its scope is
  `DEL-02-01/Design/model-document-0.7.0.md` §6.

**D3.** No file W-10 touches is in the pinned set
(`b7a_domain_model.py:74`, which pins product_physics, frame_kernel,
primitive_loads, result_export and similar files, but no applier, desktop or
schema file). W-10 reads `pressure_runtime.rs` and product_physics `lib.rs` and
edits neither. **No D3 re-pricing.**

| W-10 file | S1b | S2 (per §6) | Collision |
|---|---|---|---|
| `operation_applier/src/pipe_reconnect.rs` (new) | — | — | no |
| `operation_applier/src/lib.rs`: `mod`, `check_kinds` 2404-2486, dispatch ~1383, apply chain 2025 | version strings at other lines | adds about 14 change kinds to the same `check_kinds` lists and its own dispatch and apply arms | textual, certain with S2, trivial; none with S1b |
| `operation_applier/src/geometry_operations.rs`: `pub(crate)` on `unique`/`record_shape`/`frame` (118, 128, 318) | 661-667, 693 | `record_shape` body 128-157, scans 233-303 | S2 adjacent at 128 (one-line resolve); S1b no |
| `operation_applier/src/bend_authoring.rs`: `pub(crate)` θ helper near 154 | `:465` version check | scans 583-606, line join 631-636 | no (separate hunks) |
| `operation_applier/tests/pipe_reconnect.rs` (new) | version ripple would break a literal `0.6.0` | — | no, if fixtures use `MODEL_SCHEMA_VERSION` |
| `fixtures/model_operations/contract_corpus/case_85..87` (new) | regenerates every case | — | yes, mechanical: whichever lands second re-blesses |
| `tools/hosted-ci-routing.json` (corpus file list) | refreshes digests (§8 S1b checks) | — | possible textual, trivial |
| `schemas/model_operation.schema.json` (enum value) | — | explicitly not extended | no |
| `apps/desktop/src/types.ts` change-kind union ~1037 | line 113 only | adds kinds to the same union | S2 trivial; S1b no |
| `apps/desktop/src/features/geometry-tools/*` | — | — | no |
| `apps/desktop/src/features/toolkit/capabilityCatalog.ts` | — | may add capabilities | possible, trivial |
| `apps/desktop/src/App.tsx` (~515, pass selected node) | — | adds panels | possible, trivial |
| `apps/desktop/e2e/pipe-reconnect.spec.ts` (new) | e2e fixtures move to 0.7.0 | — | no, if the spec uses a regenerated fixture |
| `operation_applier/src/lib.rs` `resolve_delete_pipe_run` 3955-4058 (§7) | — | deleting a pipe removes it from its line; generic delete scan skips annotations (§6 of the 0.7.0 design) | textual with S2, trivial; none with S1b |

**S1b's document validator** (§5.3) is not on the branch yet. It is to run "on
every operation the applier applies". If S1b places it in the shared apply path,
reconnect's candidate passes through it with no W-10 code. Only the dispatch
hunk could be adjacent.

**Lines.** Reconnect never edits `lines`.
- Membership is by pipe id, and §3.2 sets no contiguity rule, so a reconnected
  pipe stays in its line.
- W-10's fail-closed scan skips `lines`, `unresolved_assumptions`, `constraints`
  and `equipment_interfaces` from the start. On 0.6.0 documents these keys are
  absent, so the skip is harmless, and S2 needs no reconnect change.
- If the owner later adds a contiguity rule to lines, reconnect must check it.

**Answer.** Yes. W-10 can be implemented before S1b without a D3 collision. The
only coupling with S1b is mechanical: version-neutral test fixtures, and one
corpus re-bless by whichever lands second.

**Sequencing (coordinator decision, 2026-10-23).**
- W-10 lands before S1b.
- W-10's new corpus cases (§4, §7.4) and their `hosted-ci-routing.json` list
  entries come in a follow-up after S1b. If they land first instead, S1b
  re-blesses them mechanically with the rest of the corpus; whichever lands
  second does the re-bless.
- Shared-list edits in applier `lib.rs` (`check_kinds`, dispatch, apply chain,
  `resolve_delete_pipe_run`) and in `apps/desktop/src/types.ts`: whoever merges
  second merges main and resolves.
- W-10 tests use `MODEL_SCHEMA_VERSION`, never a version literal, so S1b's
  version ripple does not touch them.

## 6. Size, risks, decisions

**Size: S–M.** About 350 lines for the Rust module, 500 for applier tests, 40
for `lib.rs` wiring, 200 for desktop code plus 200 for its tests, 150 for the
e2e spec, and 3 corpus cases; §7 adds about 60 lines of applier code and 250 of
tests.

**Verification class.** Topology and numerical meaning: the targeted regression
checks in §4 plus independent review by a separate agent (AGENTS.md,
Verification).

**Risks.** The fail-closed scan (R12) may refuse references §2.2 does not list;
that is safe, and the paths are named. A user can leave a model split or a node
orphaned; these are warnings so that edits can proceed in steps.

**Finding: reconnect refuses what connect permits.** `connect_pipe_run` has no
pair check (`lib.rs:3667-3760`), and human-accepted corpus case 08
(`case_08_accept_connect_pipe_run.json`, DEC-030) creates `pipe:P-2` N-1→N-2
beside `pipe:P-1` N-1→N-2. Coordinator decision (2026-10-23): `connect_pipe_run`
stays as it is, with no refusal and no warning, because either would change that
accepted output. Reconnect still refuses a parallel pair (R8): a new operation
carries no accepted output, and §2.1 gives the reasons (silently doubled
stiffness and weight, region refusals at solve, coincident drawing). The two
operations therefore disagree on parallel pairs by design.

**Decided scope (coordinator, 2026-10-23).** Refusing component-bearing and
pressure-region pipes (R9–R11) is acceptable for v0.1, provided each refusal
names its blocker in the UI (§6.1) and the limitation is recorded in DEL-16-06
and the release notes (§6.2). Membership-preserving reconnect inside a region
would need the chain check at authoring, which means a D3 on
`pressure_runtime.rs` or product_physics `lib.rs`, or a duplicate in the applier.
It is not provided.

### 6.1 Refusal messages

Each blocker gets its own diagnostic, so several blockers give several lines.
`affected_refs` lists the pipe first, then the blocker ids. The batch review
shows each message verbatim with its refs selectable, as for existing
diagnostics. Desktop tests assert that the message carries the blocker's id.

| Code | Message (placeholders in braces) | Remediation |
|---|---|---|
| `OP-RECONNECT-COMPONENT-ATTACHED` (names the pipe) | `Cannot reconnect {pipe}: component {component} ({kind}) names it in {slot}.` | `Delete or re-author {component}, reconnect, then author the component again.` |
| `OP-RECONNECT-COMPONENT-ATTACHED` (at a node) | `Cannot reconnect {pipe}: component {component} ({kind}) sits at node {node}, which this reconnect {leaves \| joins}.` | same |
| `OP-RECONNECT-PRESSURE-REGION-MEMBER` | `Cannot reconnect {pipe}: it is a member of pressure region {region} in load case {case}.` | `Remove {pipe} from {region}, reconnect, then re-enter the region's members and terminals.` |
| `OP-RECONNECT-PRESSURE-REGION-BRANCH` | `Cannot move the {end} end of {pipe} to {node}: {node} is an internal node of pressure region {region} in load case {case}, and a pipe outside the region cannot join it there.` | `Choose another node, or end {region} at {node} before reconnecting.` |

### 6.2 Recording text (written during implementation)

**Release notes, `docs/RELEASE_NOTES_v0.1.0_DRAFT.md`.** Replace line 57,

```
- Geometry tools: split a pipe at a new node; copy, rotate and mirror selected pipes.
```

with

```
- Geometry tools: split a pipe at a new node; reconnect one end of a pipe to another existing node; copy, rotate and mirror selected pipes.
  - Reconnect refuses a pipe that a component names or that has a component at the node it leaves or joins, a pipe in a pressure region, and a move onto a pressure region's internal node. The refusal names the blocking component or region; re-author it around the reconnect. Reconnect also refuses a second pipe between the same two nodes, which Connect pipe still allows.
```

**DEL-16-06, `ScopeOfWork.md`.** Append after the last
"Delivery commitments and evidence limits" item (DEL-16-06:3), with no new key,
because the keys index the source-retirement account:

```
- **Declared limitation (W-10, Design `Design/pipe-reconnect.md`):** `reconnect_pipe_run` refuses a pipe named by a component or with a component at the node it leaves or joins, a pressure-region member, and a move onto a region's internal node; each refusal names the blocker. Membership-preserving reconnect inside a pressure region is not provided in v0.1. `delete_pipe_run` refuses while any record names the pipe and never cascades.
```

Run `python3 -m tools.deliverables --project projects/chirality-piping check`
after the edit.

## 7. `delete_pipe_run` dangling references (defect, fixed in W-10)

### 7.1 The defect

`resolve_delete_pipe_run` (`lib.rs:3955-4058`) refuses only for primitive loads
and pressure regions (`pipe_primitive_load_references`, `:8766-8803`) and for
load/reference states (`load_state_authoring::pipe_references`, `:342-379`).
`apply_deleted_pipe` (`:7827-7839`) then removes the record. Components and wind
exposure that name the pipe are left naming a pipe that no longer exists.

### 7.2 Behaviour: refuse with named dependents

Every delete path in the applier refuses while a dependent remains; none
cascades:
- node: `OP-NODE-DELETE-REFERENCED` (`lib.rs:3486-3500`);
- support: `OP-SUPPORT-DELETE-REFERENCED` (`:6520`);
- load case: `OP-LOAD-CASE-DELETE-REFERENCED` (`:5012`);
- material, section, component: `OP-ENTITY-DELETE-REFERENCED`, whose remediation
  reads "deletion never cascades" (`:3936-3947`);
- pipe, for the slots it checks today: `OP-PIPE-DELETE-REFERENCED` (`:4042`).

Pipe delete keeps that rule. A cascade would also remove user-entered
engineering data the delete intent never shows: a bend's k and SIF pair, tee
SIFs and angles, connector stiffness, wind exposure. That data would come back
only by undo. Undo works either way (whole-model checkpoints,
`workspaceSession.ts:1192-1198`), so it does not decide the choice. A user who
wants the pipe gone removes each dependent with its own operation
(`delete_component`, the wind-exposure `update_load` at `lib.rs:829`, the region
edit). These can be composed into one reviewed batch, with the delete last.

### 7.3 Reference inventory

The delete refuses on all of the following. The list reuses the §2 inventory.

| Slot | Today | After |
|---|---|---|
| `load_cases[].primitive_loads[].target.pipe` | refused | refused |
| `load_cases[].pressure_regions[].member_pipe_ids` | refused | refused |
| `analysis_state.element_states[].pipe_ref`, `reference_configurations[].member_references[].pipe_ref` | refused | refused |
| `components[].geometry.bend_pipe_ref` | **dangles** | refused |
| `components[].geometry.branch_header_pipe_ref`, `branch_branch_pipe_ref` | **dangles** | refused |
| `components[].geometry.rigid_pipe_ref` | **dangles** | refused |
| `components[].geometry.expansion_joint_pipe_ref` | **dangles** | refused |
| `components[].objective_connector.topology.span_ref` (`lib.rs:2727-2728`) | **dangles** | refused |
| `load_cases[].equivalent_static.wind.exposed_pipe_refs[]` | **dangles** | refused |
| `load_cases[].equivalent_static.wind.exposed_spans[].pipe_ref` | **dangles** | refused |
| any other `*_ref` / `*_refs` / `ref` slot naming the pipe | **dangles** | refused (fail closed) |
| `lines[].pipe_refs` (0.7.0, S2) | — | excluded from the scan; S2 removes the pipe from its line in the same operation (0.7.0 design §3.2) |
| assumptions and constraints naming the pipe (0.7.0, S2) | — | excluded; never blocks, and S2 leaves a missing-subject warning |

**Mechanism.** After the existing two checks, run the generic delete scan
`entity_reference_paths` (`lib.rs:3818-3853`) over the model with the pipe
removed and the excluded collections skipped. This is how `resolve_delete_entity`
already scans (`:3932-3934`). The scan treats keys ending `_ref`/`_refs` and
`ref` as reference slots, so it covers every row marked "dangles". It does not
read free text (`label`, `provenance`).

**Diagnostic.** The code stays `OP-PIPE-DELETE-REFERENCED`. `affected_refs`
keeps today's refs first, in today's order, then appends each new dependent's
owning record id, sorted and deduplicated. Corpus cases 54 and 55 have no
component or wind reference to `pipe:P-1`, so their accepted outcomes (code and
refs; messages are not compared, `tests/contract_corpus.rs:19`) are unchanged
without a re-bless. The message becomes `Pipe segment {pipe} is still named by:
{owner} ({slot}), …`. The remediation becomes `Remove or retarget each
dependent first; deletion never cascades.`

### 7.4 Regression tests

- One applier test per new row in §7.3: bend, tee header, tee branch, rigid,
  expansion joint, connector span, wind exposed pipe, wind exposed span, and an
  unknown `*_ref` slot. Each asserts the code, the dependent id in
  `affected_refs` and the slot in the message, an unchanged model, and no model
  from validate mode.
- The existing refusals still hold: `lib.rs:12926` and the load-state delete
  control (`tests/load_state_delete_control.rs`).
- Corpus cases 54 and 55 pass unchanged, with no re-bless.
- A batch of [`delete_component`, `delete_pipe_run`] applies atomically, and one
  undo restores both.
- After S1b: a new corpus block case (`delete_pipe_run` named by a bend), with
  its own review entry.
- Desktop: deleting a pipe that a bend names shows the refusal naming the bend,
  and the model is unchanged.
