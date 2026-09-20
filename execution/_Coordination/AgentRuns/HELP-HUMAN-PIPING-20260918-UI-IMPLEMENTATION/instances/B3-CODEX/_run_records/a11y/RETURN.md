# B3 accessibility regression return

TASK Type 2 child `/root/b3_manager/a11y_tests`, parent `/root/b3_manager` (B3-CODEX WORKING_ITEMS), grandparent ROOT. Mechanism: Codex delegated-harness-native. Launch declares GPT-6 Astra / low; no independent model-introspection tool was available. No delegation. Written scope, not filesystem-enforced: new ordinary e2e spec plus this evidence; no product edits, Git mutations, native UI, benchmark edits, dependencies, skips, timeout changes, or colour assertions.

Sealed brief: `instances/B3-CODEX/briefs/A11Y_REGRESSIONS.md`, SHA256 ad66fd1a278958e63c2bc2d873dbeb4354c4b5547bc11fb93386a0568c3b4437. Source basis HEAD `26a7478b5b7e85760a8f9f8847690c548a01f9e5`. Applicable supplied instructions and integration/scenario records were read; actual hashes in `basis-sha256.txt`. Explicit workdir throughout: assigned swbpipe-wt3. Manager independently wrote product repairs after baseline completion; `candidate-sha256.txt` binds the tested product and final spec bytes. Manager's `PRODUCT_REPAIR.patch` is not a child-authored artifact.

## Results and exact rerun

From repository root, all browser runs used `sh projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/tools/with_e2e_lock.sh sh -c 'cd projects/chirality-piping/apps/desktop && PLAYWRIGHT_WORKERS=1 npx playwright test e2e/b3-accessibility.spec.ts --reporter=list'`. Baseline additionally used `--project=chromium-desktop`. Repaired-02 additionally set `B3_A11Y_EVIDENCE_DIR` to this evidence folder; final set it to this folder's `final-witness` subdirectory. This optional variable writes compact JSON/PNG witnesses only. Source port 5174; no dist lane. Configured widths 1440×920 and 1280×800. Node v24.18.0, npm 11.16.0, Playwright 1.60.0, Chrome 153.0.8010.48.

- `baseline.log`: three representative cases failed on unchanged source at 1440×920. Initial Results, Review and Agent tooltips clipped by ancestors and failed topmost ownership. Libraries/Project/Analyze exposed covered workspace controls in actual Tab traversal (and DOM-derived accessibility snapshot). Escape from inspector Task tab and draft input left inspector open and failed opener focus.
- `repaired-01.log`: 8 passed / 4 failed. Original geometry, Tab, wide Escape, Model and routing checks passed. DOM-derived Playwright ariaSnapshot still included inert content; HTML native-select popup attempt did not establish a consuming popup and first Escape closed inspector. Both limitations are retained, not presented as product passes.
- `repaired-02.log`: 12 passed (23.8 seconds), using actual Chrome Accessibility.getFullAXTree and a visible real toolkit popup consumer.
- `repaired-final.log`: 12 passed (23.1 seconds), after explicit retained inspector-element identity assertion and compact tooltip screenshots were added. This is the final spec revision. No later test edits.

A package-path version lookup failed because the assumed desktop node_modules path was absent; `npx playwright --version` supplied the actual 1.60.0 version. This read-only discovery failure did not affect execution.

## Issue-to-test map

1. `disabled rail and Agent reasons escape clipping under keyboard focus`: actual Tab traversal to all three controls; each reason checked against every clipping ancestor and five point hit owners. Compact JSON and Results reason PNG retained. Screenshot visually inspected; “No run yet” text is legible. No colour/contrast claim.
2. `covered pages exclude retained stage controls from Tab and accessibility`: Libraries, Project and Analyze each undergo 70 real Tabs from the rail position; no covered subtree focus. Actual Chromium AX tree excludes the workspace while covered and restores it when closed. Canvas, inspector and draft retain exact DOM identity; draft text retained and can regain focus. Inert is allowed without unmounting/hiding.
3. `Both inspector Escape closes from property tab and input to opener`: both focus origins close and focus Inspector opener at both widths. `Model inspector stays docked on Escape`: dock remains visible. `toolkit child consumes first Escape before inspector close`: actual visible toolkit dismisses and keeps inspector open, then inspector-origin Escape closes. `portalled routing Escape closes explicitly and supersedes automatic restore`: DOM-portalled input closes inspector; subsequent Select disarm cannot reopen it from previously open state.

## Limits and return

Native HTML select popup cancellation is unverified. Toolkit evidence establishes only toolkit consumed-Escape priority; it does not prove native-select behavior. No native-host behavior, narrow 1024px behavior, full browser suite, dist suite, engineering acceptance, usability, release or colour-contrast acceptance is claimed. Original baseline did not query native Chrome AX; its actual Tab finding independently reproduces the covered-surface defect, while repaired native AX exclusion is directly witnessed. Manager owns full lanes, independent review, integration and receipt.

Browser lock released after final run; no browser or native activity continues. New maintained spec SHA256: `e13713b63d526dec658b983b98ccf6a00fd30ffafe055139e91a75eaaef1fad1`. Exact artifact hashes are in `SHA256SUMS.txt`; compact baseline JSON and final witnesses are canonical, no duplicate trace tree retained.
