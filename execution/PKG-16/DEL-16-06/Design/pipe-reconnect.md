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
UI that drives it.

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

**Recommended sequencing.** Start W-10 now. Merge it after S1b if S1b merges
within W-10's window. Otherwise merge W-10 first and hold its corpus cases and
routing edit to a follow-up after S1b, so S1b's regenerated corpus set does not
move under its review. Whichever of W-10 and S2 lands second absorbs the
trivial `check_kinds`/`types.ts` conflicts.

## 6. Size, risks, open questions

**Size: S–M.** About 350 lines for the Rust module, 500 for applier tests, 40
for `lib.rs` wiring, 200 for desktop code plus 200 for its tests, 150 for the
e2e spec, and 3 corpus cases.

**Verification class.** Topology and numerical meaning: the targeted regression
checks in §4 plus independent review by a separate agent (AGENTS.md,
Verification).

**Risks.** The fail-closed scan (R12) may refuse references §2.2 does not list;
that is safe, and the paths are named. A user can leave a model split or a node
orphaned; these are warnings so that edits can proceed in steps.

**Findings outside W-10, not fixed here.**
- `connect_pipe_run` has no pair check (`lib.rs:3667-3760`). Accepted corpus
  case 08 (`case_08_accept_connect_pipe_run.json`, reviewed under DEC-030)
  creates `pipe:P-2` N-1→N-2 beside `pipe:P-1` N-1→N-2, a parallel duplicate.
- `delete_pipe_run` checks primitive loads, regions and load-state references
  only (`lib.rs:4037-4054`); a component or wind exposure naming the pipe is
  left dangling.

**Questions for the owner.** None of these changes a commitment or acceptance
criterion.
1. **Acceptance scope.** Is a reconnect that refuses component-attached pipes
   and pressure-region members (R9–R11) and directs the user to re-author them
   acceptable as FR-GUI-004's "reconnect"? If not, membership-preserving
   reconnect inside a region needs the chain check at authoring. That means
   editing the D3-pinned `pressure_runtime.rs` or product_physics `lib.rs` to
   expose it (a D3), or duplicating it in the applier (a drift risk).
2. **Parallel pipes in `connect_pipe_run`.** Should `connect_pipe_run` refuse
   parallel duplicates as R8 does? Doing so changes the expected result of
   human-accepted corpus case 08, which needs a new review entry. If not, the two
   operations disagree: reconnect refuses what connect creates. W-10 does not
   depend on the answer.
