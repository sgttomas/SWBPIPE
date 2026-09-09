# U7 I1 sealed viewport-routing implementation brief V1

- Requested by: `/root/native_authoring` (WORKING_ITEMS Agent 1)
- Parent: `/root/native_authoring`
- Child: `I1`, bounded ephemeral Agent 2 generalist
- Required runtime: `gpt-5.6-sol`, high reasoning; fresh/limited history; no delegation
- Run: `HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING`
- State: `SEALED_NOT_DISPATCHED_SOURCE_HELD`

## Objective and accepted basis

Implement the frozen orthogonal-plane routing slice exactly as defined by:

- `{RUN_ROOT}/instances/U7/ASSESSMENT_AND_FENCE_V1.md` SHA `0dd7f2433dea46cbc6ba92d3aa5f8c10e63ff6fb68344e1067ae4fed275b1252`
- `{RUN_ROOT}/instances/U7/DESIGN_AMENDMENT_V2.md` SHA `66974cc4c4773c2940c0a1abf5b903b220da99db8f7842cc0b86d1203a375719`
- repository basis HEAD `533332349a4607eee561d4ef90fb05a62d86519e`

Do not start product writes until the parent supplies the terminal design-review hash and explicit root source release. Once released, complete one coherent final source cut before broad checks.

## Allowed reads and tools

Read the repository, accepted U7 packet, terminal design review when supplied, `AGENTS.md`, `agents/AGENT_TASK.md`, the project `AGENTS.md`, software workflow profile, and existing source/tests. Use search/read, absolute-target patching, targeted Vitest, TypeScript build, and the specifically scheduled Playwright command. Do not use network, Git mutation, Rust/Cargo, native/Tauri build, unrelated Wasm build, full evidence sweep, or whole-repository tests.

## Exact allowed writes

Product/test writes are limited to:

1. `{PROJECT_ROOT}/apps/desktop/src/features/viewport/PipeViewport.tsx`
2. `{PROJECT_ROOT}/apps/desktop/src/features/viewport/viewportRouting.ts` (new)
3. `{PROJECT_ROOT}/apps/desktop/src/features/viewport/viewportRouting.test.ts` (new)
4. `{PROJECT_ROOT}/apps/desktop/src/App.test.tsx`
5. `{PROJECT_ROOT}/apps/desktop/src/styles.css`
6. `{PROJECT_ROOT}/apps/desktop/e2e/linear-authoring.spec.ts`

Evidence writes are limited to `{RUN_ROOT}/instances/U7/children/I1/**`. Do not write deliverable state, dependency/DAG records, decisions, receipts, build output, evaluation records, other source/tests, package manifests, or Git metadata.

## Required implementation

- Private helper: XY/XZ/YZ plane definitions through a resolved From node; applicable Free/X/Y/Z constraints; atomic reset to Free for an incompatible axis; a 4-CSS-pixel same-pointer gesture gate; strict three-axis engine conversion validation; and transient routing utilities.
- Pipe viewport: a transient Three construction grid, dashed route ghost, and endpoint marker without rebuilding the renderer on pointer movement. Existing-end ghost requires two resolved node IDs. New-end live ghost requires visible WebGL hover; captured ghost retains the exact constrained model-space point; manual X/Y/Z or unit edits clear pointer ghost.
- Gesture: the actual pointer event target captures the primary pointer. Same `pointerId` pointerup within 4 CSS pixels may author; movement beyond 4 pixels, pointer cancel, or lost capture cancels. OrbitControls setup/damping/sub-slop change alone cannot cancel or author.
- Units: model-space points use `model.project.units.length`; convert to the selected entered coordinate unit through the existing `convertDisplayQuantities` service in one batch. Require exactly three finite, correctly identified results in the requested unit. Conversion failure or stale completion changes no coordinates and has a visible reason.
- Workflow: plane/axis require a resolved From node and display fixed axis/elevation/model unit/From ID. Standalone node capture stays on explicit global XZ@Y=0. Typed coordinates remain authoritative. Pointer-captured IDs/labels reuse current collision-safe logic and provenance remains blank. Existing/new endpoint modes remain distinct.
- Continuation: only the accepted own commit may retain plane, applicable axis, endpoint unit, material, dimensions, y-reference, and pipe provenance; move From to the accepted end, clear consumed identities/end provenance, and keep new-end routing ready. Cancel, tool exit, external open/create/replacement, undo/redo, stale response, and invalidated async conversion clear transient route state.
- Accessibility: every disabled plane/axis/Add/pointer state has a truthful adjacent reason and matching accessible title/description. Preserve existing Add validity and busy predicates.
- Preserve exact Add -> frozen review/diff -> Apply behavior, single-operation existing-end route, atomic ordered `[create_node, connect_pipe_run]` new-end batch, hash/revision/receipt/warning protections, explicit zero values, incomplete node-only persistence, and all producer contracts.

## Required tests and checks

Add meaningful focused helper and App-level regressions listed in the frozen assessment and amendment. Do not weaken existing assertions. Run focused new/helper/App tests first. Then, when root confirms the prepared Wasm/browser slot, run from `{PROJECT_ROOT}/apps/desktop`:

- affected Vitest paths/tests, then the full affected App suite if focused tests pass;
- `npm run build`;
- `../../node_modules/.bin/playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop --project=chromium-compact`.

Playwright must visibly prove the ghost/plane/readout at 1024x768 in the test itself where required by the accepted assessment, elevated non-global-plane routing, applicable axis constraint, orbit drag non-capture, editable captured values, Add/review/Apply, and own-commit continuation. Preserve raw failure logs in child evidence; do not hide retries. Native mixed-unit/save-reopen evidence is a later root-owned gate.

## Return and escalation

Return exact changed-path hashes, full base diff and manifest, focused/full/build/browser commands and results, containment, requirement-to-test map, unresolved limits, and whether any runtime behavior outside routing changed. Stop and report before expanding the fence, editing an existing producer/API/schema/persistence path, or choosing behavior that contradicts the two frozen design documents. Do not delegate.
