# U7-I1 sealed implementation brief

RequestedBy: `/root/native_authoring`
RunID: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`
ParentInstanceID: `U7`
ChildInstanceID: `U7-I1`
Role: ephemeral Agent 2 bounded software implementer
Model: `gpt-5.6-sol`
Reasoning: `high`
Delegation: forbidden
PackageID: `PKG-07`
DeliverableID: `DEL-07-01`
SourceBasis: `55df51ac3201456e0f181823e3aefefef47a73bb`

## Objective

Implement the frozen native straight-route and inline inspector Apply slice exactly. Preserve zero coordinates, incomplete node-only create/save/reopen, exact atomic `[create_node, connect_pipe_run]` new-endpoint batch with one checkpoint, existing-endpoint single intent, validation/diff review, stale/cancel/revision/hash protections, compact palette, and persistent canvas. Queue and Validate remain in the inspector; the new Apply uses the same selected intent and App service route.

## Declared reads

- root and project `AGENTS.md`
- `agents/AGENT_WORKING_ITEMS.md`
- `docs/SOFTWARE_WORKFLOW_PROFILE.md`
- `projects/chirality-piping/software-workflow.json`
- `OWNER_ACT.md`, `SHARED_SOURCE_LAUNCH_CONTRACT_V1.md`, `WORK_GRAPH_V2.json`
- `amendments/U7/SOURCE_RELEASE_V1.md` at SHA-256 `a81ee0439c11d92360d863bc028bdd72f7f82a524220ad2dda9a348da40c1e46`
- DEL-07-01 `_CONTEXT.md`, `_STATUS.md`, and frozen `PHYSICS_UI_EXECUTION_20260908/IMPLEMENTATION_BRIEF_V1.md`
- current product and test dependencies needed to understand the nine paths

## Allowed write targets

- `{WORKING_ROOT}/apps/desktop/src/App.tsx`
- `{WORKING_ROOT}/apps/desktop/src/App.test.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/PipeViewport.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/routeDraft.ts` (new)
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/routeDraft.test.ts` (new)
- `{WORKING_ROOT}/apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/model-tree/typedInspector.test.tsx`
- `{WORKING_ROOT}/apps/desktop/src/styles.css`
- `{WORKING_ROOT}/apps/desktop/e2e/linear-authoring.spec.ts` (new)
- `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/**` for return/status only

## Allowed tools and checks

Read, write, patch, `rg`, and targeted JavaScript/TypeScript Vitest checks. Do not run WASM, native, Rust, full desktop, full harness, evidence sweep, or build while F4 owns the Rust compile slot.

## Acceptance criteria

- Only the nine product paths change.
- Route Add freezes and validates an exact current intent/batch; Apply submits only the validated frozen object.
- Any draft edit, affected selection/model change, open/create, undo/redo, or Cancel invalidates route review; delayed/stale callbacks cannot publish; repeated Apply while busy cannot duplicate.
- New endpoint emits exactly ordered `[create_node, connect_pipe_run]`; existing endpoint emits one `connect_pipe_run`; new endpoint success is one checkpoint and failure none.
- Typed coordinates remain authoritative; zero is valid; construction plane is disclosed as `XZ @ Y=0` without arbitrary-plane semantics.
- Pipe dimensions remain inline with an existing material ID; existing separate section assignment remains unchanged.
- Inspector Queue/Validate remain and direct Apply forwards the same exact current intent to App's existing application path, preserving unit, material/provenance, rule-pack, and persistence contracts.
- Meaningful route-builder, inspector, and App integration tests cover the new behavior. Add the bounded Playwright scenario without running it in this phase.
- No selective solve, model slicing, Phase B case view, component expansion, default engineering value, new gate, service/schema/backend change, or acceptance claim.

## Escalation and return

Stop and report any required write outside the fence, producer/API defect, substantive engineering choice, or test failure that cannot be fixed without weakening an accepted assertion. Return changed paths, targeted commands/results, remaining limitations, and exact handoff state. Do not commit, push, or delegate.
