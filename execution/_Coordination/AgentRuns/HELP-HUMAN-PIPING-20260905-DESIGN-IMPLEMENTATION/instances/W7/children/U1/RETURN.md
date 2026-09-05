# U1 return — persistent modeling workspace

Status: READY_FOR_MANAGER_VERIFICATION. Source frozen in SOURCE_FREEZE.json; no further edits without parent activation.
Role: Agent 2 TASK; delegated-harness-native, instruction+config asserted; actual model unknown (not exposed). No delegation.
Basis: sealed BRIEF plus parent inline v2 (instruction/run-record root), v3 (viewport presentation), v5 (host ResizeObserver). Runtime Skill software-bounded-implementation v1; three companion contracts found.

## Implemented behavior
- Persistent three-column spatial workspace: collapsible model browser, canvas, contextual properties. Browser initially collapsed at widths <=1100; properties open. Owner initial target1024 ×768; no final usability acceptance claimed.
- Visible icon and short-label Model/Loads/Analyze/Results/Rules/Report task navigation plus Select, Toolkit, Undo/Redo, counted Review. Existing engine handlers retained; queue count includes standalone intents and batch members.
- Searchable modal Toolkit floats over workspace, contains keyboard focus, closes on Escape/outside pointer, returns trigger focus, exposes unavailable reasons. Selecting a command activates required subtab, opens ancestor Details and focuses actual destination.
- Creation controls occupy independently scrolling rail beside canvas. Selection/navigation and unit-source explanations are explicit disclosures. Active creation hint concise; distinct support/load icons. ResizeObserver keeps rendered canvas/camera projection in sync with actual host dimensions when rails/dock change; skips zero sizes and disconnects on teardown.
- Single bounded lower dock. Review/Geometry/Supports/Self weight/Agent/Details navigation retains mounted forms/drafts. Agent workbench removed from permanent viewport column and remains available under Agent. Underlying operations, exact batch validation/application, session undo/redo and feature panels preserved.
- Compact project strip exposes storage action status visibly; project review details remain available. Status summaries use readable words, with raw recorded values in disclosures. Exact technical-preview notice retained.
- Shared inspector form styling integrates sibling U2 contextual forms; no U2 source edited.

## Changed files and hashes
SOURCE_FREEZE.json records eight owned source files and SHA256 digests. App.deadControls.test.tsx was not changed. Parent owns tauri.conf.json and e2e/workspace-layout.spec.ts under separate amendment.

## Verification
- build-initial.json: registered desktop-build PASS.
- build-final.json: registered desktop-build PASS, before the final narrow ResizeObserver change; manager must run latest combined build.
- _run_records/targeted-initial.log: historical123tests,13failed/110passed, intentional disclosure/copy adapters then repaired.
- _run_records/targeted-second.log:126tests,123passed/3failed; three property caption assertions overtaken by U2 copy changes.
- _run_records/workspace-routes.log:8passed/118skipped across3files; verifies12 Toolkit destinations, actual destination focus, ancestor Details expanded, expected subtab, canvas identity preserved through navigation, Escape restoration, modal focus containment, toolbar reachability/history availability.
- _run_records/copy-adapters-pass.log:3passed/117skipped after latest property-label and required-card adapters. Earlier failed focused logs retained as history.
- scope-validation.json PASS for explicit U1 changed paths/evidence; git diff --check PASS. No Git mutation or backend/model/native-app change by U1.
- Manager owns final full desktop-test/build, actual browser layout and wasm workflow evidence, and fresh independent full-diff review. This return does not claim those gates passed. Parent reported a real host/canvas-size mismatch during visual checks; v5 is the narrow repair and parent must rerun its regression check.

## Residuals and handoff
The existing field selector and Queue→Review→Apply editing sequence remain. This tranche does not implement report-wide inline Apply, new geometry semantics, analytical derivation, persistence/history redesign, live provider integration, or scope/lifecycle acceptance. Known source-level numerical risks from investigation remain under their owning packages. All warnings/results preserve existing semantics; raw audit statuses remain reachable.

No owner ruling requested by U1. Source implementation complete within brief; combined verification and independent review pending manager fan-in.
