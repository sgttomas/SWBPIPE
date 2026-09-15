# Production UI foundation — candidate next tranche

Status: prepared by HELP_HUMAN, not activated. Final readiness depends on this foundation tranche's native qualification, complete registered verification and combined acceptance. This document does not authorize production UI source changes during the result-compatibility/private-pressure tranche.

## Outcome and basis

Deliver the first production modelling workspace from the accepted 20260913 interaction design: a dominant 3D viewport, coherent model tree and selection, contextual property/task rail, discoverable tool palette, and an explicit review/apply dock. Authoring, solving, inspection, undo/redo and native persistence must remain functional throughout the layout change.

The predecessor design is the 20260913 RESULTS-ENGINEERING-3D run's accepted interaction contract and successor-v3 prototype. Its numerical outcomes and mutation traces are simulated design evidence. Production acceptance must use the actual application and shared Rust applier. This is an interface and implementation handoff, not a claim of personal visual approval or completed usability studies.

Current observed source is 8ad37207cf088025623aa1e777a97a6fcb802f48, independently reviewed through FINAL_SOURCE_RELEASE_V6.json and its V6 return in this run. Before activation, revalidate origin/main, the committed loop, deliverable Remaining entries, accepted final foundation evidence, and the actual source seams. Preserve the original local session handoff and accepted engineering repairs.

## Agent 0 orchestration

| Assignment | Responsibility | Model / reasoning |
|---|---|---|
| HELP_HUMAN / Agent 0 | Interaction priorities, file-scope arbitration, engineering boundaries, final acceptance | gpt-6-astra / high |
| WORKING_ITEMS / Type 1 | PKG07 implementation ownership, coupled UI work graph, child briefs and fan-in | gpt-5.6-sol / high |
| TASK / Type 2 → manager | One product writer for workspace composition, controls and focused tests | gpt-5.6-sol / high |
| TASK / Type 2 → Agent 0 | Fresh independent review of the complete frozen diff | gpt-6-astra / high |
| TASK / Type 2 → Agent 0 | Native, pointer, keyboard and browser lifecycle witnesses | gpt-5.6-sol / high |
| TASK / Type 2 → Agent 0 | Scoped Git/evidence closeout | gpt-5.6-sol / medium |

Use one implementation manager because the shell, viewport, palette, selection and property rail are tightly coupled. Agent 0 can resolve routine component decomposition and layout choices directly. Add HELPS_HUMANS only if a material interaction or visual-design decision exceeds the accepted design; do not add another management layer by default. Type 2 does not delegate. Stay within six active instances, use explicit ownership, serialize build/UI witnesses and Git integration, and retain actual model/parent/context attribution. After two failed repair cycles without new evidence, assign fresh Astra diagnosis.

## Bounded production sequence

1. Establish a workspace frame with stable command access, a dominant viewport, collapsible tree, contextual properties/task rail and lower review dock. Keep supporting evidence and detailed payload views reachable through disclosure without occupying the primary modelling canvas. Use the existing icon system and a consistent typography, spacing and color system.
2. Integrate camera presets, fit/orbit/pan/zoom, axes, units, construction-plane and routing-constraint controls into the viewport chrome. Preserve the current Three.js scene and authored geometry/selection behavior. Camera, density and selection changes do not dirty the model or create history. Orbit drag cannot author a point.
3. Route the accepted Build, Supports, Properties, Loads, Edit, Select and View, and Review commands through one searchable catalogue with visible keyboard focus, named controls and explicit disabled reasons. Availability must agree with a genuinely executable route. A verified stale catalogue description may be repaired; do not turn an unavailable engineering capability into a working claim.
4. Keep tree/canvas/properties on one EntityRef. Show the current entity and tool, entered values and units, applicable frame/basis and actionable validation. Preserve explicit Add/Cancel and frozen review/apply state. Each successful atomic batch creates one checkpoint; stale, duplicate, busy and cancelled work cannot mutate or restore an obsolete draft.
5. Integrate result inspection through the accepted semantic and Current/Historical interfaces. Historical remains readable without current overlays, comparisons, rule checks or readiness. Diagnostic work, counts and modes are distinct from moments and governing ratios. Record revision and mechanics-run identities stay separate.

No new engineering operation, wire contract, model/persistence version, solver formulation, daemon or agent-harness transport is introduced. All human mutation routes consume existing EditorOperationIntent/batches and the same Rust applier used by equivalent synthetic agent-authored intents.

## Initial file fence to seal at activation

Paths below are relative to WORKING_ROOT = REPO_ROOT/projects/chirality-piping, with REPO_ROOT discovered from the active checkout.

- apps/desktop/src/App.tsx and App.test.tsx
- apps/desktop/src/styles.css
- apps/desktop/src/features/workspace/WorkspaceToolbar.tsx and WorkspaceToolbar.test.tsx
- apps/desktop/src/features/workspace/ModelingWorkspace.tsx and ModelingWorkspace.test.tsx, if extracting the shell is warranted
- apps/desktop/src/features/toolkit/ToolkitPalette.tsx, ToolkitPalette.test.tsx and capabilityCatalog.ts
- apps/desktop/src/features/model-tree/ModelTree.tsx and PropertyInspector.tsx, with focused interaction tests in that directory
- apps/desktop/src/features/viewport/PipeViewport.tsx and a focused interaction test in that directory
- apps/desktop/e2e/workspace-layout.spec.ts and gui-workflow-validation.spec.ts, plus one bounded production-workspace test if required
- The activated run's evidence, accepted deliverable Remaining/MEMORY closeout and loop receipt

Read-only seams include routeDraft.ts, viewportRouting.ts, operationService.ts, operationBatchService.ts, the Rust operation applier, analysisRunCompatibility.ts, resultInterpretation.ts, resultExportAdapter.ts, HistoricalRunContext.tsx and shared types. Changes to those contracts or helpers return to Agent 0 for a documented scope amendment and affected verification; a visual reorganization is not authority for a physics or protocol change.

The manager should seal the exact subset and any necessary new test paths after live inspection. Preserve existing behavioral selectors where they still name the same action; tests should verify behavior rather than incidental markup or exact presentation wording. Do not blanket-authorize the desktop source tree.

## Acceptance

- Exercise real node and pipe creation, support/load edits, selection and property inspection, reviewed atomic application, cancellation, undo/redo, solve/inspect and persistence. Use the native cantilever lifecycle and exact human/agent-intent hash equivalence as carry-forward checks.
- Verify useful canvas and reachable task actions at 1024×768, 1280×800 and 1440×920, in compact and comfortable densities. Check keyboard focus/order, names, contrast and pointer behavior. Preserve the accepted minimum; do not label viewport-fit checks as independent usability acceptance.
- Verify no mutation from camera/view changes, no pointer placement during orbit, one checkpoint per applied batch, busy/stale/late-response guards, attachment preservation and truthful Current invalidation/Historical reopen.
- Inspect actual rendered source and distribution builds and a fresh normal native quit/reopen. Keep screenshots tied to the executing source. Prototype simulation is not production verification.
- Obtain fresh read-only software-code-review coverage of the complete source diff with no actionable finding; run the complete registered DEC-025, practitioner harness, self-check, receipt and actual candidate CI gates. Integrate through one scoped PR under standing Git authority.

## Parallel work and explicit limits

Separately scoped pressure runtime, objective connectors, numerical robustness and end-to-end sparse work may proceed alongside UI development when shared writes are controlled. They do not hold the UI foundation merely because the entire physics backlog remains open. Product-physics changes remain serialized within their owning workstream. Pressure and connector controls stay explicitly unavailable until their runtime contracts exist.

CAEPIPE export, external validation workflow implementation, harness integration, formal dependency/lifecycle promotion and a cross-session undo system remain separate. DEL07-06's held independent usability and target decisions are preserved; this tranche cannot accept them by completing an interface implementation.

Final activation readiness and the exact accepted source/interface bindings will be recorded at foundation closure. No production UI implementation is claimed here.

