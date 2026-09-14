# Interaction and state contract V1

This derivative design contract implements approved V3 §C within the prototype only. All examples and engine outcomes are simulated; their purpose is to inspect human gestures and typed operation mapping. They do not establish Rust validity, model-hash equivalence, numerical correctness or saved-product behavior. Use explicit mock revision tokens, never invented cryptographic proof.

## Workspace behavior

Persistent command groups reuse accepted vocabulary: Build, Supports, Properties, Loads, Edit, Select and View, Review. The command catalogue is discoverability and routing; only forms prepare mock intents. Each command has one current entity context, reason when disabled and source operation mapping. Search and keyboard access reach the same command. ROADMAP remains visible only as future capability information.

Tree and canvas share typed selection identity, not row index. The selection highlight and properties header show the same entity. Node pick for From/To is distinct from a blank-plane coordinate pick. Orbit drag cannot emit a point; crossing the 4 CSS pixel movement threshold cancels authoring for that gesture. Construction plane and axis constrain pointer projection; typed coordinates remain explicit authored inputs and may require changing displayed plane/constraint rather than silently moving values. A camera preset or density change cannot create history or dirty a model.

The task rail always contains the current tool, required data, explanation of blocked action and reachable Add/Cancel. Review expands in the lower dock; primary Apply is explicitly labelled simulation and tied to exactly one frozen mock revision and member list. Required input and diagnostics remain nearby; verbose payload/source evidence is optional disclosure. Collapsing the tree or dock leaves the canvas and active task accessible. Controls have visible keyboard focus and no icon-only essential command without a name.

## State machine

| From | Event | To | Allowed effect and observable evidence |
|---|---|---|---|
| Select | Tool command | Draft | Populate explicit demo/entered form; no model change |
| Draft | Pointer/typed input | Draft | Update same draft; unit and axis provenance shown |
| Draft | Add / prepare | Review | Freeze exact input payload, source set and mock basis revision |
| Review | Validate simulation | Ready or Blocked | Show simulated diagnostics and diffs; no accepted mutation |
| Ready | Apply simulation | Busy then Applied | Recheck exact basis; atomic returned mock model; one session checkpoint |
| Any draft/review | Cancel/withdraw | Cancelled then Select | Invalidate request generation; late results cannot resurrect draft |
| Review/Ready | Other model mutation | Stale | Apply unavailable; prepare from current basis |
| Busy | Conflicting apply/undo/redo | Busy | Disabled; cannot create duplicate checkpoint |
| Applied | Continue same route | Draft | From becomes matching newly applied endpoint only; retained explicit reusable fields |
| Applied | Undo/redo | Select | Restore one session checkpoint, clear Current results and invalidate pending review |
| Current results | Model change/undo/redo | Unavailable | No current overlay or current-rule result survives |
| Saved prototype record | Reopen | Historical | Historical results only; acceptance UNKNOWN; no current overlay/rule-check/readiness |
| Solve simulation | Blocked/nonconverged/cancelled | Named state | Honest status and diagnostics; never fabricated solved export |

The prototype need not implement a general engineering engine to exercise these states. Failure injection is labelled test/design simulation. Browser witnessing records actual interaction outcomes separately from the scenario's simulated numerical data.

## Typed mutation mapping

| Interaction | Shared production operation reference | Required authored context / known restriction |
|---|---|---|
| Create node | create_node | identity, XYZ and entered length unit, label, provenance |
| Route existing endpoints | connect_pipe_run | explicit From/To, pipe identity, material, section dimensions, explicit local Y, provenance |
| Route new endpoint | atomic [create_node, connect_pipe_run] | all node/pipe members; no partial publication; one checkpoint |
| Place component symbol | insert_component_symbol | kind and incidence roles; selected pipe references, geometry and source; finite insertion unavailable |
| Place/edit support | create_support / update_support | canonical family; explicit DOFs and dimensional stiffness; hanger/nonlinear fields cannot silently conflict |
| Property correction | set_field | typed target and allowed field path; exact before/after, entered unit, source |
| Split/transform | split_pipe_run / transform_pipe_run batch | explicit identities, local frame and source set; reject unsupported attached context |
| Assign shared section | assign_section / detach_section | shared reference identity; inline edits not a bypass |
| Delete | delete_node/pipe_run/support/material/section/component | reference guard, no cascade or guessed reassignment |
| Generate self-weight | create_load_case + create_primitive_load batch | selected pipes only, explicit densities/gravity, no component mass inference |
| Unit preference | display quantity query, no model operation | preserve entered source/model values; unavailable conversion explained |
| Camera/selection/result selection | view/query state, no model operation | no checkpoint or implicit acceptance |

Source mapping: apps/desktop/src/features/viewport/routeDraft.ts and viewportRouting.ts; features/toolkit/capabilityCatalog.ts; features/geometry-tools/GeometryToolsPanel.tsx; services/operationService.ts and operationBatchService.ts; core/model_operations/operation_applier/src/lib.rs::check_kinds. All paths are WORKING_ROOT-relative; exact source hashes and actual origins are in _run_records/CONTEXT_ORIGINS_V1.json.

## Results display contract

Use explicit quantity type, source kind/component, entered unit/dimension, coordinate frame, location and run basis. An identifier does not classify a quantity. Rotation is rad/angle. Only real ratios enter governing-ratio display; count/flag/mode evidence may remain dimensionless. Work residual is diagnostic N*m and not a moment or ratio; V3 does not adopt a new energy dimension/unit policy. Historical integrity interpretation remains separate from current adapter semantics. The prototype must not assert that its mock record reproduces production legacy checksum verification.

Current/Historical/Unavailable/Blocked must be distinguishable by text as well as color. Historical values can be inspected but cannot drive current overlays, rule checks or readiness. A deformation overlay is explicitly scaled/illustrative. No applicable ratio input means ratio unavailable. Pressure and connector applicability fields await the owning engineering contract; placeholder controls must say unavailable/backfill.

## Scope and dependencies

PKG07 owns visual interactions; DEL07-09 owns coverage/organization, not production implementation. PKG16 owns shared mutation semantics. This scope consumes existing typed interfaces as design references only and changes no lifecycle/dependency row. DEL07-01 E0482–E0485 remain formally unconsumed; new finite insertion/attached transforms cannot be justified by the prototype. DEL07-06 PDU045/046 independent usability and measurable targets remain held. Current layout checks retain 1024×768 native minimum and 1280×800/1440×920 larger targets, without claiming usability acceptance. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
