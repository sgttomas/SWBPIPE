# Minimum + Inspector pointer fit

Actual scope read: manager/NATIVE_FIT_REPAIR_DISPOSITION.md, SHA256 66dab35c65e755b37de9fce8e76a8821bcf30feb2c479fe606ec436c03c98ee6. Source6dd0e466f4432f21d3ce6913782b95f9a325f700 and NATIVE_REPAIR_RETURN.md are historical inputs, unchanged. Native Cancel pass is retained; native minimum pointer fit was expressly unqualified by ROOT's observations. Native screenshot25 shows whole-pane keyboard exposure; image dimensions are not logical geometry evidence. No worker native/CUA/build was performed.

## Causal measurement and bounded repair

Pinned Chromium148.0.7778.96, explicit1280×800 (configured Tauri minimum), Both + Inspector. Before maintained repair, rolegrid clientWidth420=scrollWidth420 while entity-grid client324 and implicit track420. Pointer wheel700px caused ModelTree scrollLeft84; filter x157.953125→73.953125. Intended grid scrollLeft remained0. Ancestor widths/style/rectangles, screenshots, raw output and trace: before/measurements.json, before/*.png/trace.zip, probe-before.txt.

Candidate browser-only style was exactly `.entity-grid{grid-template-columns:minmax(0,1fr)}.engineering-table [role="rowgroup"]{min-width:420px}`. It established rolegrid client324/scroll420, actual wheel scrollLeft96 and stationary filter. Candidate output remains separately labelled in candidate/ and probe-candidate.txt; it is not final product proof.

Maintained repair is those two CSS declarations plus explanatory comment. Shared grid track can shrink; existing header and VirtualList rowgroup share minimum420 content width inside coordinate-only horizontal scroller. Footer remains outside. No new wrapper, VirtualList, event handler, model/controller, pane budget or engine changes. Final maintained probe has temporaryStyles=null and repeats client324/scroll420/actual wheel96 with filter stationary; maintained/measurements.json includes exact ancestor/named geometry and screenshots/trace. probe.mjs takes optional temporary CSS only for the candidate run; no third argument was supplied to before/final maintained runs.

## Behavioral checks

All source runs use existing with_e2e_lock.sh, pinned executable recorded in measurement JSON, one Playwright worker, fixed5174. New test in existing B4 spec uses each configured viewport once, actual mouse wheel and pointer Z edit/Cancel; no programmatic scroll setter, explicit-size loop or tolerance/budget change.

- Fail-before new fit: `npx playwright test e2e/b4-table-editing.spec.ts --grep 'B4 Both with Inspector' --workers=1 --output <fail-before-artifacts>`: both profiles failed rolegrid scrollLeft oracle; fail-before.txt and traces retained.
- Same unchanged fit test after maintained repair: both profiles pass, repaired-fit.txt/artifacts. Proves rolegrid actual pointer scroll, zero ModelTree horizontal scroll, stationary filter/type/footer, unchanged pane/canvas rectangles during scroll, aligned header/body/Z, pointer access to valid Z Cancel with no checkpoint, and type-tab reachability. Compact project is1280×800.
- Prior B4 cases: `npx playwright test e2e/b4-table-editing.spec.ts --grep 'B4 node coordinates|B4 virtualized invalid editor' --workers=1 --output <prior-b4-artifacts>`: four pass, prior-b4.txt.
- Exact legacy three titles ×two profiles: ui-foundation Grid-drafts, r2-smoke guided-workbench and b3a persisted-marker; six pass, legacy-02.txt/artifacts. Initial legacy.txt used an anchored historical selector and selected zero tests; retained failure, then only CLI anchors removed. No test oracle changed.
- `npx tsc --noEmit`: exit0, typescript.txt. No extra unit/full-suite execution for this CSS-only product repair; existing interaction assertions are exercised in prior B4 journeys. `git diff --check` passes.

Screenshots inspected: native-repair/25-keyboard-Z-editor.png and repaired-fit compact screenshot. New fit screenshot visibly keeps filter/type controls contained while Z is exposed. This is source-browser evidence only; rebuilt actual Tauri pointer-fit recheck and independent backcheck remain manager/ROOT responsibilities. Existing native Cancel result is not extended to a fit claim. No acceptance/release/full-B4 claim.

CLEANUP.json verifies no5174 listener, no shared lock, no exact pinned-browser root process after browser.close and owned-server process-group cleanup. Browser slot released.
