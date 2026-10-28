# Pipe reconnect (W-10, FR-GUI-004)

**Basis:** `origin/main` at `356a9be374` (2026-10-23); W-13 branches
`origin/claude/piping-model-07-s1b-20261023` (`0d20039cac`, with the S1b-V
document validator) and
`origin/claude/piping-model-states-20261023` (`f0cbc15bb6`). Revision 2, after
a Codex review and an independent Claude review (§8). Implemented in W-10
(`core/model_operations/operation_applier/src/pipe_reconnect.rs`); where
implementation review changed a rule, the section states the as-built rule
(§2.2, §3, §7). Paths are relative to `projects/chirality-piping/`, except
`tools/hosted-ci-routing.json` at the repository root. Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

FR-GUI-004 (Must) asks for add, modify, delete, move and reconnect while schema
validation is preserved. Everything except reconnect exists (completeness map,
FR-GUI-004 row). A pipe's `from`/`to` is read-only in the table
(`apps/desktop/src/features/model-tree/ModelTree.tsx:1281-1282`) and is not a
field rule of `Element` (`core/model_operations/operation_applier/src/lib.rs:408-470`).
This deliverable owns the applier (`deliverable.yaml` code path
`core/model_operations/operation_applier/**`); DEL-07-01 owns the geometry-tools
UI. W-10 also fixes a `delete_pipe_run` defect that leaves names of a deleted
pipe behind (§7).

Notation: P is the reconnected pipe; F its fixed end node; O the end node it
leaves; T the node it joins.

## 1. The operation

**Name.** `reconnect_pipe_run`, operation kind `modify`, the same row as
`split_pipe_run` and `transform_pipe_run` (`lib.rs:2450-2452`).

**Wire.** As the geometry wire (`geometry_operations.rs:668-677`): `target` =
(`Element`, P), `field_path` `pipe_segments`, unit `none`, dimension
`dimensionless`. `change.before` is the canonical JSON of P's current record, as
for split (`geometry_operations.rs:370`). `change.after`:

```json
{ "pipe_ref": "pipe:P-3", "end": "to", "target_node_ref": "node:N-9",
  "y_reference": { "x": 0, "y": 0, "z": 1 } }
```

- `pipe_ref` equals `target.ref`. `end` is `from` or `to`; F does not move and P
  keeps its from→to sense. Unknown fields are refused (`fields()`,
  `geometry_operations.rs:27-41`).
- `y_reference` is optional. Present: it replaces P's in the same change.
  Absent: P keeps its own; if P has none, it stays absent and the warning
  `OP-RECONNECT-ORIENTATION-MISSING` says the solve will refuse
  `PIPE_ORIENTATION_INPUT_MISSING` (`product_physics/src/lib.rs:8050-8060`). The
  applier never derives an orientation.

**No new-node mode.** Reconnecting to a new point is the batch
[`create_node`, `reconnect_pipe_run`]; the atomic batch validates each step
against a fresh hash (`atomic_batch.rs:118-125`).

**Identity (FR-MOD-003).** No id changes. P's record is replaced in place (array
order kept). Only the moved end and, optionally, `y_reference` are written.

**Fit in the applier.** New module `operation_applier/src/pipe_reconnect.rs`,
called only from the common applier.
- `check_kinds` gains the kind (`lib.rs:2404-2486`). `run_with_context`
  dispatches next to the geometry branch (`lib.rs:1383-1410`) and requires
  claimed model-hash evidence as split and transform do (`lib.rs:1384-1396`).
- **Return type.** `GeometryEdit` has no warnings (`geometry_operations.rs:9-12`)
  and the geometry branch pushes only blocking errors (`lib.rs:1404-1408`). The
  module returns `ReconnectEdit { model, diff_preview, warnings:
  Vec<ReconnectWarning { code, message, affected_refs }> }`, after the precedent
  `RichEdit.warnings` (`rich_authoring.rs:13-18`), but typed so each warning
  keeps its code and refs. Errors are `GeometryError` (code, message, refs).
  The dispatch pushes each warning with severity `warning` in **both** validate
  and apply mode, so preview shows exactly what apply will report; warnings
  never block.
- The candidate joins the apply chain at `lib.rs:2025-2028`.
- Reused helpers: `record_shape`, `unique`, `references`, `diff`, `global_ids`
  (`geometry_operations.rs:128,118,233,337,195`); the private ones become
  `pub(crate)`. `frame` is not reused for orientation (§2.3).

**Diff.** One row (P, `Element`, `pipe_segments`, before record, after record,
`reconnect_pipe_run`), the shape split uses (`geometry_operations.rs:409-417`).
Validate mode returns diff and diagnostics, never a model (`lib.rs:885-905`).

**Undo.** The applier has no undo. Desktop undo/redo restores a whole-model
checkpoint per applied batch (`apps/desktop/src/features/workspace/workspaceSession.ts:1192-1198`).
In applier tests, "undo" means the inverse reconnect: same P and `end`, target
O, and P's original `y_reference` when the forward change replaced it. The
inverse is representable only when P had a `y_reference` before the change or
the forward change did not add one: `y_reference` is optional on a pipe
(`geometry_operations.rs:144`), omission in the payload keeps the current value,
and no field rule removes it. A forward change that adds `y_reference` to a pipe
that had none therefore has no applier inverse; its restoration is the desktop
checkpoint Undo. The wire is not extended to express removal.

**Runtimes.** The desktop runs the same crate as wasm
(`apps/desktop/src/services/operationService.ts:14`); there is no TypeScript
mirror. DEL-16-06 CLM-003 still needs a runtime-equivalence witness in the W-10
PR (§4).

**Agent proposals and the capability reference.** Offline intake reaches the
same batch path; preflight requires the exact intent shape
(`atomic_batch.rs:285-300,355-380`). Reconnect writes no user-only datum
(unlike k, `lib.rs:1366-1381`), so an agent may propose it. The schema
`schemas/model_operation.schema.json` is **not** an executable reference: its
`OperationChange` (`target_ref`, `value_payload`, `:316-330`) differs from the
intake shape. Therefore:
- The capability reference handed to the intake panel (`App.tsx:507-511`) gains
  `operation_examples.reconnect_pipe_run`: the wire constants above and one
  complete batch for the open model, generated by the same builder the panel
  uses. Selection: candidate batches (pipe, end, target) in deterministic order,
  each run through the shared engine's batch validation (the same wasm
  `validate_operation_batch` the panel uses), and the first with no blocking
  diagnostic is published. The search is bounded: at most 32 candidates, run on
  demand when the intake panel opens or the user asks for the reference (never
  on render), and cached against the model hash. When the cap is reached with no
  pass, the example is omitted with the reason "no valid reconnect found in the
  first 32 candidates". This covers every refusal, R13 included, so an
  imported load with an unsupported extent field (permissive decoder,
  `product_physics/src/lib.rs:780-797`) cannot yield a published example that
  then refuses. If no candidate passes, the example is omitted with a stated
  reason.
- The schema's `change_kind` enum gains `reconnect_pipe_run` beside split and
  transform (`:337-338`), as documentation only. Its existing generic
  `reconnect` operation kind (`:421`) and change kind (`:346`) stay; the applier
  keeps refusing them (`OP-KIND-UNSUPPORTED`, `lib.rs:2436-2447`), and their
  retirement is not part of W-10.

## 2. Rules

Two kinds of change are separated:
- **Connectivity** changes at O (loses P) and T (gains P).
- **Geometry** changes at F and T: P's direction changes at both, and its length
  changes. A node move (`Node` `position.*`, `lib.rs:386-406`) already changes
  geometry through the generic field path with no attachment check, so a
  geometry change alone is not a refusal; it refuses only where a declared
  record depends on P's direction (components) or the result is numerically
  invalid.

### 2.1 Refusals (blocking; model unchanged)

| # | Condition | Code |
|---|---|---|
| R1 | P absent, or `pipe_ref` ≠ target, or payload malformed | `OP-TARGET-NOT-FOUND` / `OP-RECONNECT-PAYLOAD-INVALID` |
| R2 | `before` ≠ P's current canonical record | `OP-STALE-BEFORE-VALUE` (`lib.rs:7608-7628`) |
| R3 | T absent | `OP-RECONNECT-TARGET-NODE-NOT-FOUND` |
| R4 | T = O | `OP-RECONNECT-NO-CHANGE` |
| R5 | T = F | `OP-RECONNECT-ZERO-LENGTH` |
| R6 | New chord degenerate or nonfinite (kernel `local x axis` error) | `OP-RECONNECT-GEOMETRY-DEGENERATE` |
| R7 | Kernel orientation refuses the new chord with the kept or entered `y_reference` (§2.3) | `OP-RECONNECT-ORIENTATION-DEGENERATE` |
| R8 | — (now a warning, §2.4) | — |
| R9 | A component names P, or a component other than `kind: "terminal"` has `node` ∈ {F, O, T}, or names F, O or T in any other slot | `OP-RECONNECT-COMPONENT-ATTACHED` |
| R10 | P is in any `pressure_regions[].member_pipe_ids` | `OP-RECONNECT-PRESSURE-REGION-MEMBER` |
| R11 | T is an internal node of a region: an end of a member pipe or member connector span that is not one of its terminals | `OP-RECONNECT-PRESSURE-REGION-BRANCH` |
| R12 | The scan (§2.2) finds P, F, O or T in a slot not listed as allowed | `OP-RECONNECT-ATTACHMENT-UNSUPPORTED` |
| R13 | A pipe-id hit sits in an allowed slot whose record is not of the exact supported shape (§2.2) | `OP-RECONNECT-ATTACHMENT-UNSUPPORTED` |

**Why R9 covers F.** The solver applies a bend or tee component's SIF and
flexibility to every pipe endpoint at `component.node`
(`product_physics/src/lib.rs:12719-12730,12775-12784`). A geometry-only bend
marker (angle, SIF, flexibility, no `bend_pipe_ref`) at F would keep applying to
P after P's direction at F changed, so its declared angle could go stale while
passing a check that looked only at O and T. Consistent with the decided rule
that component-bearing reconnects refuse, a component at F refuses too. Named
components: a bend (`geometry.bend_pipe_ref`; arc from chord, radius and
`y_reference`, `bend_authoring.rs:109-124`); a tee (`branch_header_pipe_ref`,
`branch_branch_pipe_ref`, node at an end of both legs, `lib.rs:2831-2869`); a
rigid component (`rigid_pipe_ref`); an annotation expansion joint
(`expansion_joint_pipe_ref`); an objective connector
(`objective_connector.topology.span_ref`, type `replaces_span`,
`lib.rs:2727-2728`). SIF and flexibility modifiers are quantities that name no
pipe (`schemas/model.schema.yaml:435-484`); they matter only through
`component.node`. `terminal` markers have no solver contribution (schema
`ComponentKind` description, `:404`), so they are exempt from the at-node rule:
a pipe whose end carries only a terminal marker can be reconnected (the marker
stays on its node), and a pipe can join a node that carries one.

**Why R10–R11.** Region chain topology is checked only at solve
(`traverse_region`, `pressure_runtime.rs:1458-1597`); authoring checks members,
terminals and bend tangency, not degree, branching or cycles
(`pressure_authoring.rs:141-221`). `pressure_runtime.rs` is D3-pinned
(`tools/retained_pricing/base/b7a_domain_model.py:74`), so the applier refuses
instead of re-running the chain: every member reconnect, and every move onto an
internal node (which the solve refuses as `PRESSURE_REGION_INTERNAL_BRANCH`,
`:1541-1549`). Moving a non-member pipe away from an internal node removes a
branch and is allowed.

### 2.2 The scan

One scanner, `references()` (`geometry_operations.rs:231-250`), for both
reconnect and the §7 delete. It reports every string exactly equal to a given id
anywhere in the document, except in the whole subtrees under `label`, `name`
and `provenance`. For pipe and node ids it finds every slot the key-name matching
in `entity_reference_paths` (`lib.rs:3818-3853`) finds, and also `target.pipe`,
`member_pipe_ids` and `node`. It is not a strict superset: a `*_ref` nested inside
an object-valued `provenance` is found only by `entity_reference_paths`. Pipe and
node ids do not occur there (provenance records cite sources, not model
entities), so the difference is harmless for these scans.
The root `diagnostics` array (`schemas/model.schema.yaml`, root member) is
skipped: stored diagnostics are derived records, not dependents. P's own record
and the `nodes[]` records of F, O and T are skipped as self-references.

Two scans run, with separate allowed slots. Paths are written as
`references()` builds them; `*` is any index.

**As built.**
- **Structured paths.** The scanner carries structured path segments
  (`reference_segments`, `geometry_operations.rs`); `references()` renders
  them only for messages. Paths are never re-parsed, so an object key spelled
  like path syntax (`"primitive_loads[0].target"`) is a single key.
- **Wildcards.** In the allowed-slot patterns, `*` matches an array index only,
  never an object key named `*`.
- **R9 exemption.** The exemption for R9's own records covers `components`
  followed by an index, and nothing else.
- **Malformed collections** (`malformed_collections`). A collection that the
  checks read as an array is never read as empty when it holds another value;
  reconnect refuses under R12. This covers:
  - at the root: `nodes`, `pipe_segments`, `supports`, `components`,
    `load_cases`, `reference_configurations`;
  - per load case: `primitive_loads`, `pressure_regions`,
    `analysis_state.element_states`, `equivalent_static.wind.exposed_pipe_refs`,
    `equivalent_static.wind.exposed_spans`;
  - per region: `member_pipe_ids`, `member_connector_ids`, `terminals`;
  - per reference configuration: `member_references`.

  An explicit `null` in one of these slots also refuses; the solve already
  refuses such a document (`LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED`).
- **Malformed items.** A non-string item inside a region's `member_pipe_ids`
  is dropped by the R11 region read, not refused. That document is
  schema-invalid, and the solve's typed decoder refuses it.

**Pipe id P — allowed slots (kept):**

| Path | Exact supported record shape | Warning |
|---|---|---|
| `load_cases[*].primitive_loads[*].target.pipe` | record keys exactly ⊆ {`id`, `category`, `target`, `direction`, `magnitude`, `dimension`, `provenance`}; `target` exactly {`type`: `"element"`, `pipe`}; `category` ∈ {`distributed_force`, `thermal`}; the shape `resolve_create_primitive_load` writes (`lib.rs:5420-5437`) | `OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED` for `distributed_force` |
| `load_cases[*].equivalent_static.wind.exposed_pipe_refs[*]` | string item | `OP-RECONNECT-WIND-EXPOSURE-REMAPPED` |
| `load_cases[*].equivalent_static.wind.exposed_spans[*].pipe_ref` | record keys ⊆ {`pipe_ref`, `start_fraction`, `end_fraction`} (schema `WindExposedSpan`) | `OP-RECONNECT-WIND-EXPOSURE-REMAPPED` |
| `load_cases[*].analysis_state.element_states[*].pipe_ref` | as `load_state_authoring` reads it (`:368-377`) | none |
| `reference_configurations[*].member_references[*].pipe_ref` | keys {`pipe_ref`, `basis`, `fit`, `provenance`} (schema `MemberReference`) | `OP-RECONNECT-MEMBER-FIT-REVIEW` when `fit.kind` ≠ `none` |

Refused with a specific code: `components[*]…` (R9) and
`load_cases[*].pressure_regions[*].member_pipe_ids[*]` (R10). Any other path, or
an allowed path whose record fails its exact shape (R13), refuses with the path
named.

R13 exists because the product decoder does not deny unknown primitive or target
fields (`PreviewPrimitiveLoad`, `product_physics/src/lib.rs:780-797`, no
`deny_unknown_fields`) and turns every element target into a uniform
whole-element load (`:9891-9905`), while the desktop open check does not
validate primitive records (`modelDocumentShape.ts:72-85`). An imported
distributed load carrying, say, a partial extent would keep those bytes while
solving over the full new span; reconnect refuses it rather than carry an
unsupported shape across a geometry change. W-10 does not harden the decoder.

**Node ids F, O, T — allowed slots:**

| Path | Treatment |
|---|---|
| `pipe_segments[*].from`, `.to` | topology (the reconnect itself) |
| `supports[*].node` | kept on its node |
| `load_cases[*].primitive_loads[*].target.node` | kept on its node |
| `load_cases[*].pressure_regions[*].terminals[*].node_ref` | kept; at O or T, warning `OP-RECONNECT-PRESSURE-TERMINAL-REVIEW` (a non-member pipe leaves or joins a terminal whose `closure_transfer` the user declared) |
| `components[*].node` with `kind: "terminal"` | kept |
| `components[*]` anything else | R9 |

Any other node-id hit refuses (R12).

**0.7.0 dependency (as built with W-13 S2a).** `record_shape` accepts `tags`
and `notes` on pipes and nodes, and the scan (`reference_segments`) skips the
`tags`, `notes` and `provenance_record` annotations wherever they occur. The
scan does not read the root model-data collections `lines`,
`unresolved_assumptions`, `constraints`, `equipment_interfaces` and
`rule_pack_refs` (0.7.0 design §3.2–§3.6, §6). Lines hold pipes by id with no
contiguity rule, so a reconnected pipe stays in its line. Assumptions,
constraints and equipment interfaces never block; a missing subject is the
document validator's warning. The same skips apply to `delete_pipe_run`'s
dependent check (§7), which shares this scan.

### 2.3 Orientation (R6, R7)

`geometry_operations::frame` (`:318-336`) only checks that the y-projection is
nonzero, which is weaker than the kernel: `FrameOrientation::new` requires unit
axes, pairwise orthogonality within 1e-12 and a right-handed basis
(`core/solver/frame_kernel/src/lib.rs:589-612`), and the solve builds every
straight element's frame through `from_x_axis_and_y_reference` (`:617-630`, via
`FrameElement::orientation`, `:695-697`). Counterexample: chord (1, 1, 0) with
kept `y_reference` (1.00001, 1, 0) passes `frame` (perpendicular norm about
7.07e-6) but the kernel's |x·y| is about 4.4e-11 > 1e-12.

So the module calls `FrameOrientation::from_x_axis_and_y_reference` on the new
chord in metres and the effective `y_reference`. Each endpoint position is
converted to metres and then subtracted, `to - from`, the order the solve uses
(node positions normalized to metres, then `FrameElement::local_x_axis`), so the
chord is the solve's chord bit for bit. The predicate is
`pipe_reconnect::kernel_orientation`; `connect_pipe_run` applies the same one
(O4, DEL-07-02 `run-sheet-entry.md` §8) under its own codes
`OP-CONNECT-PIPE-GEOMETRY-DEGENERATE` (R6) and
`OP-CONNECT-PIPE-ORIENTATION-DEGENERATE` (R7).

This needs a direct dependency on `open_pipe_stress_frame_kernel`
in the applier's `Cargo.toml`. That is not a D3-pinned file (the pinned list,
`b7a_domain_model.py:74`, holds `.rs` sources; using the kernel's public API
edits none of them). The crate is already in the wasm engine's closure through
product_physics, so the engine's crate set is unchanged. The applier
`Cargo.toml` digest in `tools/hosted-ci-routing.json:283` is refreshed in the
W-10 PR. The new dependency also changes the four lockfiles that record the
applier's dependencies, which numerical CI builds with `--locked`
(`tools/ci/numerical_ci.py:414-444`): `core/model_operations/operation_applier/Cargo.lock`,
`apps/desktop/src-tauri/Cargo.lock`, `core/loads/self_weight_wasm/Cargo.lock`
and `validation/benchmarks/physics_audit_regression/Cargo.lock`. They are in
W-10's scope; none is D3-pinned.

Mapping, by `FrameKernelError` variant (`frame_kernel/src/lib.rs:322-336`), not
message text: `NonFiniteInput { name: "local_x_axis" }` and `DegenerateAxis`
from the chord normalization → R6; `NonFiniteInput { name: "y_reference" }`,
`DegenerateAxis` from the y or z normalization, and `InvalidOrientation` → R7.
The kernel's `normalize` is private, so the module first runs its own chord
check on the same metre vector at the same threshold (norm ≤ 1e-12 or
nonfinite → R6) and calls `from_x_axis_and_y_reference` only on a chord that
passes it. Any `DegenerateAxis` from that call therefore comes from the y or z
normalization and maps to R7; the sources are told apart by call order, not by
`detail`. Missing `y_reference` → chord check only, plus the §1 warning. Because the chord is formed in the solve's order through the same unit conversion,
the applier and the solve's frame constructor admit the same chords and
orientations; the solve refuses any other by name (`PIPE_ELEMENT_INPUT_INVALID`, `product_physics/src/lib.rs`), never a silent result.

### 2.4 Warnings (non-blocking; the user may be mid-edit)

| Code | Condition |
|---|---|
| `OP-RECONNECT-PARALLEL-PIPE` (was R8) | another pipe already joins the same unordered node pair {F, T} |
| `OP-RECONNECT-COINCIDENT-SPAN` | another pipe's two endpoint positions coincide with F's and T's positions (either order) within 1e-9 × the new span length, through different node ids |
| `OP-RECONNECT-NODE-ORPHANED` | O has no pipe left; the node and its attachments, including any `terminal` marker, are kept and listed |
| `OP-RECONNECT-MODEL-SPLIT` | the pipe graph has more connected parts than before |
| `OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT` | T now joins three or more pipes (R9 guarantees no non-terminal component there; a `terminal` marker at T is named in the warning) |
| `OP-RECONNECT-BEND-JUNCTION-NOT-TANGENT` | F or T is an end of a realized bend arc not refused under R9, and P meets the arc there at θ > α_tan |
| `OP-RECONNECT-ORIENTATION-MISSING` | P has no `y_reference` after the change |
| slot warnings | as listed in §2.2 |

- **Parallel and coincident spans.** Neither the solver nor `connect_pipe_run`
  checks pairs; the build adds each pipe as its own element
  (`product_physics/src/lib.rs:7980-8170`). Parallel members can be intended:
  they can carry different materials, temperatures and pressure regions, with
  per-element stress recovery. They also double stiffness and weight if
  unintended, and two in one region are refused at solve
  (`PRESSURE_REGION_TOPOLOGY_INVALID`, `pressure_runtime.rs:1535-1540,1594-1596`).
  Reconnect therefore warns and does not refuse, which also agrees with
  `connect_pipe_run`. **Guarantee:** the warnings cover exact node-pair
  duplicates and full-span geometric coincidence within the tolerance; partial
  collinear overlap is not detected.
- **Orphan.** An unconnected, unrestrained node makes the solve refuse with
  `NUMERICAL_INTEGRITY_UNRESOLVED` (a zero original diagonal at the node) and
  publish no result; it is not reported as a witnessed mechanism. §4 pins this
  case. The
  desktop offers to delete O in the same batch (§3). A cycle is allowed with no
  diagnostic.
- **Bend tangency.** Insert-at-corner places the bend component at the arc's
  `from` node (`bend_authoring.rs:638-664`), which R9 refuses at F, O or T. So
  with product-authored bends the warning fires when F or T is an arc's `to`
  node; for externally authored placements it fires at any arc end not already
  refused. θ comes from `junction` (`bend_authoring.rs:135-158`, either
  endpoint, through a new `pub(crate)` helper) and is evaluated at both F and T.
  Outside regions the solve does not refuse a kink; inside a region it is a
  mitre (`pressure_runtime.rs:1655-1683`).
- **Junction.** On an exact document admission is by component kind
  (`exact_admission.rs:205-206,284`), so an untee'd junction is not refused
  there either.
- No diagnostic: local axes follow the new chord and `y_reference`; results in
  local axes change orientation; thermal expansion scales with the new length;
  generated self-weight compares section-only dependencies
  (`self_weight.rs:375,941`), so it is expected to stay Fresh (§4 pins it).

## 3. Desktop UX

**Where.** Geometry tools gains a `reconnect` action
(`apps/desktop/src/features/geometry-tools/GeometryToolsPanel.tsx:59`,
`geometryDraft.ts:10`): one source pipe (existing picker and "Use selected
pipes"); `End` (`from` | `to`, each showing its current node); `Target node`
(picker, or a new "Use selected node"); optional `y_reference`. "To a new point"
adds the `create_node` fields and queues the two-operation batch. The panel
states the §2 rules in one paragraph (as `:56` does for split/transform). The
toolkit gains `build.reconnect`, routed to `geometry-tools`
(`features/toolkit/capabilityCatalog.ts:63-71,390`).

**Not in v1.** No viewport drag or snap (viewport drag orbits,
`PipeViewport.tsx:2388,2554`; node picking exists only in the create-pipe draft,
`:1703-1716,1947-1950`). From/To stay read-only in the table, since a cell edit
would bypass batch review.

**Preview gating (any batch containing a reconnect).** Today Apply is enabled
without a prior Validate (`features/toolkit/BatchReviewPanel.tsx:48-49`;
`workspaceSession.ts:1170-1198` validates or applies directly). Batches reach
the queue from several producers through one handler,
`handleQueueOperationBatch` (`workspaceSession.ts:1137-1158`): the geometry
builder, offline intake of an arbitrary parsed batch
(`OfflineProposalIntakePanel.tsx:37`), and requeue of retained intents
(`App.tsx:458`). The gate is therefore derived centrally, not from the
producer:
- `handleQueueOperationBatch` sets `requiresPreview` on the desktop-side
  `QueuedBatch` (not the batch wire, whose fields preflight fixes) when any
  submitted operation has `change.change_kind === "reconnect_pipe_run"`.
- One predicate, `previewSatisfied(entry, outcome, revision)`, holds when
  `requiresPreview` is false, or when `outcome` is a validation-only outcome for
  `entry.key` with no blocking diagnostic, its `initial_model_hash` equals
  `entry.basisHash`, and `entry.basisRevision === revision` (`:42`).
- `BatchReviewPanel` disables Apply when the predicate fails, and the apply
  path of `handleRunOperationBatch` (`workspaceSession.ts:1160`) checks the same
  predicate and refuses with a message before calling the engine, so no other
  caller can bypass the button.

**As built.**
- **Key only.** `handleRunOperationBatch` takes only the key from its caller.
  It runs the registered submission, which is deep-frozen at queueing and held
  in a live registry (a ref, not render-time state).
- **Whether a preview is required** is derived from the registered batch's
  operations; the `requiresPreview` flag can only add the requirement.
- **Preview binding.** A successful validation records the registered batch's
  canonical SHA-256. A gated apply recomputes it and refuses if it is absent or
  differs.
- **Withdrawal.** Every queue withdrawal (clear, new, open or replaced project)
  revokes all registrations synchronously and advances a queue epoch. The run
  re-checks its registration after each await, so a handler retained from
  before a withdrawal refuses.
- **The queue epoch** is separate from the request epoch, which every
  selection change advances, so selecting an entity does not strand a
  previewed batch.

**Single-intent apply.** `handleApplyIntent` (`workspaceSession.ts:1438-1449`,
reached from `App.tsx:425,551,818` and `workspaceSession.ts:1529-1536`) applies
one intent with no preview. It refuses any intent whose `change.change_kind` is
`reconnect_pipe_run`, returns `false` with the message "Reconnect is applied
through batch review; it has been queued there.", and passes the intent to
`handleQueueOperationBatch` as a one-operation batch, where the gate applies.
`handleApplyDraftReview` is already gated and is unchanged.

The batch is immutable under its key; a changed batch is a new key with no
outcome. Batches without a reconnect are unaffected; general gating would change
every existing apply flow and its tests, so it is left out.

**Diagnostic details (all batches).** `BatchOutcome` shows only message and
remediation (`BatchReviewPanel.tsx:74,79`). It gains each diagnostic's code and
`affected_refs`, with refs selectable through the existing selection. This is
additive for other batch users; tests that match the whole line text update.

**Orphan cleanup.** If O would have no pipe and `node_references`
(`lib.rs:8618-8700`) finds nothing else (pipes, supports, `component.node`,
region terminals, nodal primitive loads — the only slots it checks), the panel
offers "Also delete node O" (unchecked), appending `delete_node`, which refuses
on the same `node_references` set (`lib.rs:3486-3500`). Other node references
(for example an equipment interface after S2) are outside that check.

**Refusal display.** Each blocker is its own diagnostic: `affected_refs` lists P
first, then the blocker ids. Messages:

| Code | Message | Remediation |
|---|---|---|
| `OP-RECONNECT-COMPONENT-ATTACHED` (names P) | `Cannot reconnect {pipe}: component {component} ({kind}) names it in {slot}.` | `Delete or re-author {component}, reconnect, then author the component again.` |
| `OP-RECONNECT-COMPONENT-ATTACHED` (at a node) | `Cannot reconnect {pipe}: component {component} ({kind}) sits at node {node}, where this reconnect {changes the pipe's direction \| removes the pipe \| adds the pipe}.` | same |
| `OP-RECONNECT-PRESSURE-REGION-MEMBER` | `Cannot reconnect {pipe}: it is a member of pressure region {region} in load case {case}.` | `Remove {pipe} from {region}, reconnect, then re-enter the region's members and terminals.` |
| `OP-RECONNECT-PRESSURE-REGION-BRANCH` | `Cannot move the {end} end of {pipe} to {node}: {node} is an internal node of pressure region {region} in load case {case}, and a pipe outside the region cannot join it there.` | `Choose another node, or end {region} at {node} before reconnecting.` |
| `OP-RECONNECT-ATTACHMENT-UNSUPPORTED` | `Cannot reconnect {pipe}: {id} is named at {path}, which reconnect does not carry across a geometry change.` | `Remove or re-author that record, then reconnect.` |

Undo/redo restores the whole-model checkpoint. During review the viewport
highlights P, F, O and T through the existing selection presentation.

## 4. Tests

**Applier rules** (`tests/pipe_reconnect.rs`; inline models from
`MODEL_SCHEMA_VERSION`, never a version literal):
- One test per refusal R1–R7, R9–R13: code, refs, unchanged model, empty diff,
  and no model from validate mode.
- R9 witnesses: a geometry-only bend marker at F (no `bend_pipe_ref`); a tee at
  O; a connector naming P by `span_ref`; a `terminal` marker at T (allowed).
- §2.3: the near-parallel counterexample refused as R7; missing `y_reference`
  warned; a kept `y_reference` that becomes parallel refused; an entered one
  accepted.
- Each §2.2 allowed slot kept byte-identically with its warning; each slot with
  an extra key or a non-element target refused as R13; an unknown slot naming P
  and one naming F refused as R12; a stored `diagnostics` entry naming P
  ignored.
- Each §2.4 warning appears exactly when its condition holds, including the
  coincident span through a distinct node and a realized arc's `to` node at F
  and at T.
- Identity: id sets and array order unchanged; only the moved end and optional
  `y_reference` differ. Agent and user authors give the same outcome.
- Property-style (deterministic seeds, no new dependency): random graphs of 3–12
  nodes, every (P, end, T). Each outcome is a refusal with no mutation, or an
  application with all endpoints existing, no zero-length pipe, id sets
  unchanged, and, where the inverse is representable (§1 Undo), the inverse
  reconnect restoring an equal canonical hash; warning presence matches an
  independent recomputation.
- Batches: [`create_node`, `reconnect`] and [`reconnect`, `delete_node`] atomic;
  a late refusal rolls back (pattern `atomic_batch.rs:609-652`).

**Numerical scrutiny.** Canonical equality with a directly authored model is
kept, within each solver mode, but only as an identity check. The physical
expectations are stated independently and every solve asserts its status:
In the cantilever, wind and fit tests O would otherwise be left orphaned and
unrestrained (the solve then refuses it as `NUMERICAL_INTEGRITY_UNRESOLVED`
with no result), so each applies the batch [`reconnect`,
`delete_node` O]; the built-topology test anchors O instead.
- **Cantilever.** P anchored at F, uniform `distributed_force` w on the
  pressure-free (Euler–Bernoulli) route, reconnected from O to a farther T.
  Assert tip deflection w·L⁴/(8·E·I) and anchor reaction w·L with L = |T − F|,
  relative tolerance 1e-9, in `DenseScrutiny` and `SparseInteractive`.
- **Built topology.** From the result envelope, P's element ends are F and T,
  and no element names O (O kept with a full anchor so the model solves).
- **Wind span.** `exposed_spans` fractions (a, b) on P: the generated wind
  resultant equals the per-length wind force × (b − a) × L_new, compared against
  an independently authored model of the final geometry.
- **Fit.** P between two anchors with `natural_length_change` Δ: axial force
  E·A·Δ/L_new in magnitude, against an independently authored model.
- **Local frame.** After a reconnect that rotates P's axis, the solve succeeds
  and reports local results; the near-parallel case does not reach the solve.
- **Orphan.** Orphaning an unsupported node gives a blocking
  `NUMERICAL_INTEGRITY_UNRESOLVED` refusal and no result
  (`tests/pipe_reconnect.rs`, `orphaning_an_unsupported_node_refuses_the_solve_without_a_result`).
- **Exact pressure.** A document whose region fails
  `PRESSURE_REGION_INTERNAL_BRANCH` (`pressure_runtime.rs:1541-1549`) because a
  non-member pipe ends at an internal node; reconnecting that pipe away gives a
  solved status. Also a non-member reconnect onto a terminal (warning, solves).

**Runtime equivalence in the W-10 PR** (CLM-003; corpus cases wait for S1b).
A native test and a wasm vitest (`apps/desktop/src/services/`) build the same
inline model and reconnect batch from their language's version constant
(Rust `MODEL_SCHEMA_VERSION`; TypeScript `MODEL_SCHEMA_VERSION`,
`apps/desktop/src/services/projectService.ts:55` today, `modelSchemaVersion.ts`
after S1b) and each assert the same codes, diff rows, warnings
and applied document, stated in both tests; one accept and one R9 refusal.

**Corpus (after S1b).** One accept case and two block cases (R9, R10), blessed
with `CORPUS_BLESS=1`, each needing its own human review entry (corpus README),
listed under `contract_corpus.rs` in `tools/hosted-ci-routing.json:431-`.

**Desktop and e2e.**
- `geometryDraft.test.ts`: payload, two-operation batch.
- `workspaceSession` tests: `requiresPreview` is set for a reconnect batch from
  the geometry builder, from offline intake and from retained-intent requeue,
  and not for a batch without a reconnect; the apply path refuses an
  unpreviewed reconnect batch even when called directly; other kinds apply as
  today.
- `handleApplyIntent` with a `reconnect_pipe_run` intent returns `false`, leaves
  the model unchanged and queues a gated batch; another intent kind applies as
  today.
- Checkpoint Undo restores a pipe that had no `y_reference` after a reconnect
  that added one (canonical hash equal to the original).
- `GeometryToolsPanel.test.tsx`: fields, "Use selected node", orphan checkbox.
- `BatchReviewPanel` tests: reconnect Apply disabled before Validate, after a
  blocking Validate and after a model change; enabled after a clean Validate;
  other batches unchanged; codes and refs shown.
- `workflows.test.tsx`: the capability reference's reconnect example passes
  actual offline intake against a fixture model and validates. Selection
  witness: on a model whose first candidate pipe carries an element load with an
  unsupported extra field (R13), the published example names a different pipe
  and still validates; with no passing candidate, the example is omitted with
  its reason.
- `ToolkitPalette.test.tsx`: the new capability.
- New `apps/desktop/e2e/pipe-reconnect.spec.ts` (after
  `b4-table-editing.spec.ts:7-50`): reconnect, Validate, Apply, check From/To in
  the table, undo, redo; then a component-attached pipe shows the refusal naming
  the component, with the model unchanged.

## 5. Overlap with W-13 S1b and S2

**Branch facts.**
- **S1b** (`origin/claude/piping-model-07-s1b-20261023` at `0d20039cac`,
  including the S1b-V document validator, `61641e541f`/`248d243d1d`). In the
  applier it changes version literals in `geometry_operations.rs:661-667,693`
  and `lib.rs` (expansion-joint and primitive-load messages, tests), plus
  `joint_pressure_authoring.rs`, `pressure_authoring.rs`,
  `boundary_association.rs` and `b7b_transport_tests.rs` (none of them W-10
  files). It adds `document_validator.rs` with its `mod` lines and a hook in
  `run_with_context` after the apply chain (`lib.rs` hunk at about 2230 on the
  branch). It adds a `[dev-dependencies]` section to the applier `Cargo.toml`,
  rewrites the applier package's dependency list in
  `operation_applier/Cargo.lock`, and changes the applier `Cargo.toml` digest at
  `tools/hosted-ci-routing.json:283` and the applier source list (about line
  2372, adding `document_validator.rs`). It regenerates all 84 corpus cases and
  the D3 chain. It does not touch `bend_authoring.rs` or the other three
  lockfiles.
- **States branch** (`origin/claude/piping-model-states-20261023`): design
  document only.
- **S2:** not written; scope is the 0.7.0 design §6.

**D3.** W-10 edits no pinned file (`b7a_domain_model.py:74` pins product_physics,
frame_kernel, primitive_loads, result_export and similar `.rs` sources, no
applier, desktop, schema or `Cargo.toml` file). W-10 reads `pressure_runtime.rs`,
product_physics `lib.rs` and the frame kernel and edits none. **No D3
re-pricing.**

| W-10 file | S1b | S2 (0.7.0 §6) | Collision |
|---|---|---|---|
| `operation_applier/src/pipe_reconnect.rs` (new) | — | — | no |
| applier `lib.rs`: `mod`, `check_kinds`, dispatch ~1383, apply chain ~2025, `resolve_delete_pipe_run` 3955-4058 | version strings elsewhere | new kinds in the same lists; delete removes the pipe from its line | textual with S2, trivial |
| `geometry_operations.rs`: `pub(crate)` on `unique`, `record_shape` (118, 128) | 661-667, 693 | `record_shape` body, scans | S2 adjacent at 128 |
| `bend_authoring.rs`: θ helper near 154 | — | scans 583-606, line join 631-636 | no |
| applier `Cargo.toml` (frame-kernel entry in `[dependencies]`) | adds `[dev-dependencies]` directly after `[dependencies]` | — | yes, adjacent hunks; whoever lands second resolves |
| `core/model_operations/operation_applier/Cargo.lock` | rewrites the applier package's dependency list | — | yes; whoever lands second regenerates it |
| `Cargo.lock` in `apps/desktop/src-tauri`, `core/loads/self_weight_wasm`, `validation/benchmarks/physics_audit_regression` | — | — | no |
| `tests/pipe_reconnect.rs` (new) | version ripple | — | no (constant, not literal) |
| corpus cases (new, after S1b) | regenerates every case | — | mechanical re-bless, see sequencing |
| `tools/hosted-ci-routing.json`: applier `Cargo.toml` digest (`:283`), applier source list (add `pipe_reconnect.rs` near `:2372`), corpus list after S1b | changes the same digest and adds `document_validator.rs` to the same source list | — | yes; whoever lands second recomputes the digest and merges the list |
| `schemas/model_operation.schema.json` (enum value) | — | not extended | no |
| `apps/desktop/src/types.ts` change-kind union | line 113 only | new kinds in the same union | S2 trivial |
| `features/geometry-tools/*`, `features/toolkit/BatchReviewPanel.tsx` | — | — | no |
| `features/toolkit/capabilityCatalog.ts`, `App.tsx` (capability example, selected node) | — | may add entries or panels | possible, trivial |
| `apps/desktop/e2e/pipe-reconnect.spec.ts` (new) | e2e fixtures move to 0.7.0 | — | no, with a regenerated fixture |

**S1b's document validator** (0.7.0 design §5.3) runs in `run_with_context`
after the apply chain whenever an applied model exists, and refuses or warns on
model-data slots only. Reconnect's batch simulation runs each step in Apply mode
(`apply_operation_in_batch`, `lib.rs:1304-1317`), so the batch review shows its
findings. It checks only non-physics slots, which a reconnect does not change,
so it adds no reconnect-specific refusal. Each reconnect step will also carry any
pre-existing document warnings (for example `MODEL-ASSUMPTION-SUBJECT-MISSING`);
the review shows them with the reconnect's own warnings. A single-operation
validate would not show them, because it returns no applied model; reconnect is
applied only through batch review (§3), so this does not affect it.

**Answer.** W-10 can be implemented before S1b without a D3 collision.

## 6. Size, risks, decisions

**Size: M.** About 450 lines for the Rust module, 700 for applier tests, 60 for
`lib.rs` wiring, 300 for desktop code (panel, gating, diagnostic details,
capability example) plus 350 for its tests, 150 for the e2e spec, 3 corpus cases
after S1b, and §7's 60 lines of code with 250 of tests.

**Verification class.** Topology and numerical meaning: §4's targeted checks
plus independent review by a separate agent (AGENTS.md, Verification).

**Risks.** The fail-closed scans (R12, R13) can refuse records a user expects to
carry over; the refusal names the path. Model splits and orphans are warnings,
so edits can proceed in steps.

**Finding: connect and reconnect on parallel pipes.** `connect_pipe_run` has no
pair check (`lib.rs:3667-3760`), and human-accepted corpus case 08
(`case_08_accept_connect_pipe_run.json`, DEC-030) creates `pipe:P-2` N-1→N-2
beside `pipe:P-1`. Coordinator decision: `connect_pipe_run` stays unchanged, with
no refusal and no warning, because either would change that accepted output.
Reconnect, a new operation with no accepted output, warns on parallel and
coincident spans (§2.4) and does not refuse; the two operations now agree on
admission and differ only in that reconnect warns.

**Decided scope (coordinator, 2026-10-23).** Reconnect refuses, for v0.1:
- a pipe that a component names (bend span, tee leg, rigid component, expansion
  joint, connector span);
- a pipe with a non-terminal component at its fixed end, at the node it leaves,
  or at the node it joins;
- a pipe that is a member of a pressure region;
- a move onto an internal node of a pressure region.

Each refusal names its blocker in the UI (§3), and the limitation is recorded as
below. Membership-preserving reconnect inside a region would need the chain
check at authoring (a D3 on `pressure_runtime.rs` or product_physics `lib.rs`,
or a duplicate in the applier); it is not provided.

**Records.** The limitation is recorded in the release notes and DEL-16-06 with
these texts.

Release notes, `docs/RELEASE_NOTES_v0.1.0_DRAFT.md` line 57. Old:

```
- Geometry tools: split a pipe at a new node; copy, rotate and mirror selected pipes.
```

New:

```
- Geometry tools: split a pipe at a new node; reconnect one end of a pipe to another existing node; copy, rotate and mirror selected pipes.
  - Reconnect refuses a pipe that a component names (bend span, tee leg, rigid component, expansion joint or connector span), a pipe with a component other than a terminal marker at either of its ends or at the node it joins, a pipe in a pressure region, and a move onto a pressure region's internal node. The refusal names the blocking component or region; re-author it around the reconnect.
  - Deleting a pipe is refused while any record still names it; the refusal lists each dependent.
```

DEL-16-06 `ScopeOfWork.md`. Old (last lines of "Delivery commitments and
evidence limits"):

```
- **DEL-16-06:3** — Preserve all engineering, privacy, professional, lifecycle and release holds named by the accepted instruments.
```

New (the same line, then one keyless line; the keys index the source-retirement
account):

```
- **DEL-16-06:3** — Preserve all engineering, privacy, professional, lifecycle and release holds named by the accepted instruments.
- **Declared limitation (W-10, Design `Design/pipe-reconnect.md`):** `reconnect_pipe_run` refuses a pipe named by a component (bend span, tee leg, rigid component, expansion joint, connector span), a pipe with a non-terminal component at its fixed end or at the node it leaves or joins, a pressure-region member, and a move onto a region's internal node; each refusal names its blocker. Membership-preserving reconnect inside a pressure region is not provided in v0.1. `delete_pipe_run` refuses while any record names the pipe and never cascades.
```

Then run `python3 -m tools.deliverables --project projects/chirality-piping check`.

**Recorded gap, not fixed in W-10.** The generic entity delete
(`resolve_delete_entity`, `lib.rs:3932-3947`) scans the root `diagnostics` too,
so a stored diagnostic whose `affected_refs` names a component, material or
section blocks deleting it. The same latent issue would affect any delete that
scans stored diagnostics; the W-10 scans skip them.

## 7. `delete_pipe_run` dangling references (defect, fixed in W-10)

**Defect.** `resolve_delete_pipe_run` (`lib.rs:3955-4058`) refuses only for
primitive loads and pressure regions (`pipe_primitive_load_references`,
`:8766-8803`) and for load/reference states
(`load_state_authoring::pipe_references`, `:342-379`); `apply_deleted_pipe`
(`:7827-7839`) then removes the record. Components and wind exposure naming the
pipe are left naming a pipe that no longer exists.

**Behaviour: refuse with named dependents.** Every applier delete refuses while
a dependent remains; none cascades: node (`lib.rs:3486-3500`), support
(`:6520`), load case (`:5012`), material, section and component
(`OP-ENTITY-DELETE-REFERENCED`, remediation "deletion never cascades",
`:3936-3947`), and pipe for the slots it checks today (`:4042`). A cascade would
also remove user-entered data the delete intent never shows (a bend's k and SIF
pair, tee SIFs and angles, connector stiffness, wind exposure). Undo works
either way (whole-model checkpoints), so it does not decide. A user removes each
dependent with its own operation (`delete_component`, the wind-exposure
`update_load`, the region edit), optionally as one reviewed batch with the
delete last.

**Scan.** After the two existing checks, the §2.2 scanner `references()` runs
for the pipe id over the model, skipping the pipe's own record and the root
`diagnostics`. Hits at the slots the existing checks already report
(`primitive_loads[*].target.pipe`, `pressure_regions[*].member_pipe_ids[*]`,
`element_states[*].pipe_ref`, `member_references[*].pipe_ref`) are dropped, so
each dependent is reported once. Every remaining hit refuses. This covers, among
others:

| Slot | Today | After |
|---|---|---|
| `components[*].geometry.bend_pipe_ref` | dangles | refused |
| `components[*].geometry.branch_header_pipe_ref`, `branch_branch_pipe_ref` | dangles | refused |
| `components[*].geometry.rigid_pipe_ref`, `expansion_joint_pipe_ref` | dangles | refused |
| `components[*].objective_connector.topology.span_ref` | dangles | refused |
| `load_cases[*].equivalent_static.wind.exposed_pipe_refs[*]`, `exposed_spans[*].pipe_ref` | dangles | refused |
| any other slot naming the pipe | dangles | refused (fail closed) |
| `lines`, assumptions, constraints (0.7.0) | — | S2 adds them: deleting a pipe removes it from its line in the same operation; assumptions and constraints never block (0.7.0 design §3.2, §6) |

**As built.** A malformed collection (§2.2, as built) is reported as a named
dependent, for example `case:w (pressure_regions, not an array)`. Delete
therefore refuses whenever one is present, even if no readable record names the
pipe.

**Owner mapping.** Each path maps to its owning record: the first
`collection[i]` segment gives the record (its `id`); for paths inside
`load_cases[i]` the owner is the load case id, with the nested slot named. For
example `components[0].geometry.bend_pipe_ref` → `component:B-1
(geometry.bend_pipe_ref)`.

**Diagnostic.** Code unchanged, `OP-PIPE-DELETE-REFERENCED`. `affected_refs`
keeps today's refs first and in today's order, then appends the new owners,
sorted and deduplicated. Corpus cases 54 and 55 contain no component or wind
reference to `pipe:P-1`, so their accepted outcomes (code and refs; messages are
not compared, `tests/contract_corpus.rs:19`) are unchanged with no re-bless.
Message: `Pipe segment {pipe} is still named by: {owner} ({slot}), …`.
Remediation: `Remove or retarget each dependent first; deletion never cascades.`

**Regression tests.**
- One applier test per new slot: bend, tee header, tee branch, rigid, expansion
  joint, connector span, wind exposed pipe, wind exposed span, and an unknown
  slot. Each asserts the code, the owner id in `affected_refs`, the slot in the
  message, an unchanged model and no model from validate mode.
- A pipe named by both an element state and a component is reported once per
  owner.
- A stored `diagnostics` entry naming the pipe does not block.
- Existing refusals hold (`lib.rs:12926`, `tests/load_state_delete_control.rs`);
  corpus cases 54 and 55 pass unchanged.
- [`delete_component`, `delete_pipe_run`] applies atomically and one desktop
  undo restores both.
- After S1b: a corpus block case (`delete_pipe_run` named by a bend), with its
  own review entry.
- Desktop: deleting a pipe that a bend names shows the refusal naming the bend,
  with the model unchanged.

## 8. Review findings resolved

Reviews of `d55b17cfb3` (and §7 at `6cc3ef6dba`): Codex and an independent
Claude review, both "sound with changes".

| Finding | Resolution |
|---|---|
| Codex 1 (P1): fixed end F missing from geometry checks | Adopted. §2 separates connectivity (O, T) from geometry (F, T); R9 covers components at F; tangency warning at F and T; witnesses in §4. Verified: node-based modifiers `product_physics/src/lib.rs:12719-12784`. |
| Codex 2 (P1): no preview gating; codes and refs not shown | Adopted, adapted: gating for reconnect batches only (desktop `requiresPreview`), code and refs display for all batches (additive). §3, §4. |
| Codex 3 (P2): `frame` weaker than the kernel | Adopted. Kernel API through a direct applier dependency; no pinned file edited; mapping and missing-Y behaviour stated. §2.3. |
| Codex 4 (P2): preserved-slot shapes; node-id scan | Adopted. Exact shapes and R13; separate pipe-id and node-id allowed slots. §2.2. |
| Codex 5 (P2): coincident span through a distinct node | Adopted, superseded by the coordinator's later decision: R8 is a warning too, with the coincident-span warning and its tolerance; the categorical "no modelling use" claim is removed. §2.4. |
| Codex 6 (P2): equality is not numerical scrutiny | Adopted with Claude 1: independent expectations, explicit status asserts, native/wasm witness in the W-10 PR. §4. |
| Codex 7 (P3): enum value is not an executable reference | Adopted with Claude 9: a generated complete batch in the capability reference, tested through offline intake. §1, §4. |
| Claude 1: equality is an identity check only | Adopted (cantilever wL⁴/8EI, built topology, wind span, fit, region branch). §4. |
| Claude 2: two scanners, unclear fail-closed scope | Adopted. One scanner (`references()`) for reconnect and delete; allowed slots as paths. §2.2, §7. |
| Claude 3: commitment and release text need the owner's words | Adopted: the DEL-16-06 and release-note lines are proposed as exact old → new text, applied only after approval. §6. |
| Claude 4: parallel pipes can be intended | Adopted: R8 became a warning (coordinator decision). §2.4. |
| Claude 7: terminal markers block free-end reconnects | Adopted: `terminal` markers are exempt from the at-node rule. §2.1. |
| Claude 5: stored diagnostics | Adopted: skipped by both scans; the same latent issue in the generic entity delete recorded as a gap. §6. |
| Claude 6: R9 field names | Adopted: `objective_connector.topology.span_ref`; SIF modifiers name no pipe. §2.1, §7. |
| Claude 8: `GeometryEdit` has no warnings | Adopted: `ReconnectEdit` with typed warnings, shown in validate mode. §1. |
| Claude 9: generic `reconnect` in the schema | Adopted: stays, stays refused, points to `reconnect_pipe_run`; not retired in W-10. §1. |
| Claude 10: overlap facts | Adopted: S1b touches neither `bend_authoring.rs` nor the root routing file on the branch; its other applier edits listed; 0.7.0 §3.2; `record_shape` dependency; S2 adds the skips. §2.2, §5. |
| Claude 11: `delete_node` claim too broad | Adopted: narrowed to the `node_references` slots. §3. |
| Claude 12: "undo" in applier tests | Adopted: inverse reconnect, or desktop checkpoint restore. §1, §4. |
| Claude 13: bend warning precision | Adopted: component at the arc's `from` node (R9); warning at an arc's `to` node, at F and T. §2.4. |
| Claude §7 (a): stored diagnostics block deletion | Adopted: both scans skip the root `diagnostics`. §2.2, §7. |
| Claude §7 (b): path → owner id | Adopted. §7 owner mapping. |
| Claude §7 (c): de-duplication | Adopted: hits already reported by the existing checks are dropped. §7. |
| Codex r2-1 (P1): gating depended on the producer | Adopted. `requiresPreview` derived centrally in `handleQueueOperationBatch` from the batch contents; the same predicate enforced in the apply path; tests for builder, imported and requeued batches. Verified `OfflineProposalIntakePanel.tsx:37`, `App.tsx:458`, `workspaceSession.ts:1137-1158`. §3, §4. |
| Codex r2-2 (P2): inverse cannot restore an absent `y_reference` | Adopted. Inverse property limited to representable inverses; absence→present restored by checkpoint Undo, tested; no wire extension. Verified `geometry_operations.rs:144`. §1, §4. |
| Codex r2-3 (P3): example selection omitted R13 | Adopted. The example is the first candidate batch that passes shared-engine validation; unsupported-shape selection witness. §1, §4. |
| Claude r2-0 (Claude 3): record approval | Adopted: records applied only after the owner approves; the PR quotes the owner's words for the DEL-16-06 line. §6. |
| Claude r2-1: `handleApplyIntent` applies without preview | Adopted: it refuses a reconnect intent and routes it to the gated batch path; test added. Verified `workspaceSession.ts:1438-1449` and its call sites. §3, §4. |
| Claude r2-2: lockfiles | Adopted: the four lockfiles are in scope; none is D3-pinned. §2.3, §5. |
| Claude r2-3: orphaned O in numerical tests | Adopted: [`reconnect`, `delete_node` O] in the cantilever, wind and fit tests; O anchored in the topology test. §4. |
| Claude r2-4: unbounded example search | Adopted: cap of 32, on demand, cached by model hash, stated reason at the cap. §1. |
| Claude r2-5: nits | Adopted: terminal markers named in the orphan and junction warnings; "no non-terminal component"; the scanner claim reworded; kernel errors mapped by variant and call site. §2.2–§2.4. |
| Coordinator §5 update (S1b at `0d20039cac`) | Adopted: overlap re-checked; `Cargo.toml`, applier `Cargo.lock` and routing digest and source list shared, resolved by whoever lands second; validator behaviour stated; chord-check wording. §2.3, §5. |
| Coordinator: `terminal` markers | Adopted: exempt from the at-node rule. §2.1. |
| Coordinator: records | Adopted: proposed as exact old → new text, applied only after approval. §6. |
